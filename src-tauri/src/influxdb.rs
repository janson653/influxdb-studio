use crate::models::{
    ConnectionProfile, InfluxDBVersion, InfluxDBV1Config, InfluxDBV2Config,
    QueryResult, DatabaseInfo, Series
};
use crate::error::AppError;
use reqwest::Client;
use serde_json::Value;

/// InfluxDB 服务枚举
pub enum InfluxDBService {
    V1(InfluxDBV1Service),
    V2(InfluxDBV2Service),
}

impl InfluxDBService {
    pub async fn ping(&self) -> Result<bool, AppError> {
        match self {
            InfluxDBService::V1(service) => service.ping().await,
            InfluxDBService::V2(service) => service.ping().await,
        }
    }

    pub async fn query(&self, query: &str) -> Result<QueryResult, AppError> {
        match self {
            InfluxDBService::V1(service) => service.query(query).await,
            InfluxDBService::V2(service) => service.query(query).await,
        }
    }

    pub async fn query_with_database(&self, query: &str, database: &str) -> Result<QueryResult, AppError> {
        match self {
            InfluxDBService::V1(service) => service.query_with_database(query, database).await,
            InfluxDBService::V2(service) => service.query_with_database(query, database).await,
        }
    }

    pub async fn get_databases(&self) -> Result<Vec<String>, AppError> {
        match self {
            InfluxDBService::V1(service) => service.get_databases().await,
            InfluxDBService::V2(service) => service.get_databases().await,
        }
    }

    pub async fn get_database_info(&self, database: &str) -> Result<DatabaseInfo, AppError> {
        match self {
            InfluxDBService::V1(service) => service.get_database_info(database).await,
            InfluxDBService::V2(service) => service.get_database_info(database).await,
        }
    }

    pub async fn get_measurements(&self, database: &str) -> Result<Vec<String>, AppError> {
        match self {
            InfluxDBService::V1(service) => service.get_measurements(database).await,
            InfluxDBService::V2(service) => service.get_measurements(database).await,
        }
    }
}

/// InfluxDB v1.x 服务实现
pub struct InfluxDBV1Service {
    client: Client,
    config: InfluxDBV1Config,
    base_url: String,
}

impl InfluxDBV1Service {
    pub async fn new(config: InfluxDBV1Config) -> Result<Self, AppError> {
        tracing::info!(
            "Creating InfluxDB v1 service for host: {}:{}",
            config.host,
            config.port
        );
        let protocol = if config.use_ssl { "https" } else { "http" };
        let base_url = format!("{}://{}:{}", protocol, config.host, config.port);
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))?;
        
        Ok(Self {
            client,
            config,
            base_url,
        })
    }

    async fn ping(&self) -> Result<bool, AppError> {
        tracing::info!("Pinging v1 service at {}", self.base_url);
        let url = format!("{}/ping", self.base_url);
        let response = self.client.get(&url).send().await
            .map_err(|e| AppError::Network(e.to_string()))?;
        
        if response.status().is_success() {
            Ok(true)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "N/A".to_string());
            let error_message = format!(
                "Connection failed with status: {}. Response: {}",
                status, text
            );
            tracing::warn!("{}", error_message);
            Err(AppError::Network(error_message))
        }
    }

    async fn query(&self, query: &str) -> Result<QueryResult, AppError> {
        // 使用配置中的默认数据库
        self.query_with_database(query, &self.config.database).await
    }

    async fn query_with_database(&self, query: &str, database: &str) -> Result<QueryResult, AppError> {
        tracing::info!("[BE] InfluxDBV1Service::query_with_database called with query: '{}', database: '{}'", query, database);
        
        // 检查是否是 INSERT 语句
        let upper_query = query.to_uppercase();
        if upper_query.starts_with("INSERT") {
            return self.write_with_database(query, database).await;
        }
        
        let start = std::time::Instant::now();
        
        let url = format!("{}/query", self.base_url);
        tracing::info!("[BE] Making HTTP request to: {}", url);
        
        let mut request_builder = self.client
            .get(&url)
            .query(&[("q", query)]);
        
        // 添加数据库参数
        request_builder = request_builder.query(&[("db", database)]);
        tracing::info!("[BE] Added database parameter: {}", database);
        
        // 添加认证信息
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            request_builder = request_builder.basic_auth(username, Some(password));
            tracing::info!("[BE] Added basic auth for user: {}", username);
        }
        
        let response = request_builder
            .send()
            .await
            .map_err(|e| {
                tracing::error!("[BE] HTTP request failed: {}", e);
                AppError::Network(e.to_string())
            })?;

        let status = response.status();
        tracing::info!("[BE] HTTP response status: {}", status);
        
        let response_text = response.text().await.map_err(|e| {
            tracing::error!("[BE] Failed to read response text: {}", e);
            AppError::Network(e.to_string())
        })?;
        
        tracing::info!("[BE] Response text: {}", response_text);
        
        if status.is_success() {
            let series = self.parse_query_response(&response_text)?;
            let execution_time = start.elapsed().as_millis() as u64;
            
            tracing::info!("[BE] Query succeeded, parsed {} series, took {}ms", 
                          series.len(), execution_time);
            
            Ok(QueryResult {
                series,
                execution_time,
            })
        } else {
            let error_message = format!("HTTP {status}: {response_text}");
            tracing::error!("[BE] Query failed: {}", error_message);
            Err(AppError::Query(error_message))
        }
    }

    /// 处理 INSERT 语句，使用 /write 端点
    async fn write(&self, insert_query: &str) -> Result<QueryResult, AppError> {
        // 使用配置中的默认数据库
        self.write_with_database(insert_query, &self.config.database).await
    }

    /// 处理 INSERT 语句，使用指定的数据库
    async fn write_with_database(&self, insert_query: &str, database: &str) -> Result<QueryResult, AppError> {
        tracing::info!("[BE] InfluxDBV1Service::write_with_database called with query: '{}', database: '{}'", insert_query, database);
        let start = std::time::Instant::now();
        
        // 解析 INSERT 语句，提取数据部分
        let data_line = self.parse_insert_query(insert_query)?;
        
        let url = format!("{}/write", self.base_url);
        tracing::info!("[BE] Making HTTP write request to: {}", url);
        
        let mut request_builder = self.client
            .post(&url)
            .query(&[("db", database)])
            .body(data_line);
        
        // 添加认证信息
        if let (Some(username), Some(password)) = (&self.config.username, &self.config.password) {
            request_builder = request_builder.basic_auth(username, Some(password));
            tracing::info!("[BE] Added basic auth for user: {}", username);
        }
        
        let response = request_builder
            .send()
            .await
            .map_err(|e| {
                tracing::error!("[BE] HTTP write request failed: {}", e);
                AppError::Network(e.to_string())
            })?;

        let status = response.status();
        tracing::info!("[BE] HTTP write response status: {}", status);
        
        let response_text = response.text().await.map_err(|e| {
            tracing::error!("[BE] Failed to read write response text: {}", e);
            AppError::Network(e.to_string())
        })?;
        
        tracing::info!("[BE] Write response text: {}", response_text);
        
        if status.is_success() {
            let execution_time = start.elapsed().as_millis() as u64;
            
            tracing::info!("[BE] Write succeeded, took {}ms", execution_time);
            
            // 返回空的查询结果，表示写入成功
            Ok(QueryResult {
                series: vec![],
                execution_time,
            })
        } else {
            let error_message = format!("HTTP {status}: {response_text}");
            tracing::error!("[BE] Write failed: {}", error_message);
            Err(AppError::Query(error_message))
        }
    }

    /// 解析 INSERT 语句，提取数据行
    fn parse_insert_query(&self, insert_query: &str) -> Result<String, AppError> {
        // 支持多种 INSERT 语法：INSERT INTO "db" data 或 INSERT "db" data 或 INSERT data
        let upper_query = insert_query.to_uppercase();
        
        if !upper_query.starts_with("INSERT") {
            return Err(AppError::Validation("INSERT 语句必须以 'INSERT' 开头".to_string()));
        }
        
        // 使用更简单的方法：直接查找数据行的开始位置
        if upper_query.starts_with("INSERT INTO") {
            // INSERT INTO "database" measurement,tag_key=tag_value field_key="field_value"
            // 或 INSERT INTO database measurement,tag_key=tag_value field_key="field_value"
            
            // 跳过 "INSERT INTO "
            let after_insert_into = &insert_query[12..].trim();
            
            // 查找数据库名和数据行的分界点
            let data_start = if after_insert_into.starts_with('"') {
                // 数据库名被引号包围，查找第二个引号后的内容
                let mut quote_count = 0;
                let mut data_start_pos = 0;
                
                for (i, ch) in after_insert_into.chars().enumerate() {
                    if ch == '"' {
                        quote_count += 1;
                        if quote_count == 2 {
                            data_start_pos = i + 1;
                            break;
                        }
                    }
                }
                
                if quote_count < 2 {
                    return Err(AppError::Validation("数据库名引号不匹配".to_string()));
                }
                
                let data_part = &after_insert_into[data_start_pos..].trim();
                if data_part.is_empty() {
                    return Err(AppError::Validation("INSERT 语句缺少数据行".to_string()));
                }
                data_part.to_string()
            } else {
                // 数据库名没有引号，查找第一个空格后的内容
                if let Some(first_space) = after_insert_into.find(' ') {
                    let data_part = &after_insert_into[first_space..].trim();
                    if data_part.is_empty() {
                        return Err(AppError::Validation("INSERT 语句缺少数据行".to_string()));
                    }
                    data_part.to_string()
                } else {
                    return Err(AppError::Validation("INSERT 语句缺少数据行".to_string()));
                }
            };
            
            Ok(data_start)
        } else {
            // INSERT "database" measurement,tag_key=tag_value field_key="field_value"
            // 或 INSERT database measurement,tag_key=tag_value field_key="field_value"
            // 或 INSERT measurement,tag_key=tag_value field_key="field_value"
            
            // 跳过 "INSERT "
            let after_insert = &insert_query[7..].trim();
            
            if after_insert.is_empty() {
                return Err(AppError::Validation("INSERT 语句缺少数据行".to_string()));
            }
            
            // 检查是否包含数据库名
            if after_insert.starts_with('"') {
                // 数据库名被引号包围，查找第二个引号后的内容
                let mut quote_count = 0;
                let mut data_start_pos = 0;
                
                for (i, ch) in after_insert.chars().enumerate() {
                    if ch == '"' {
                        quote_count += 1;
                        if quote_count == 2 {
                            data_start_pos = i + 1;
                            break;
                        }
                    }
                }
                
                if quote_count < 2 {
                    return Err(AppError::Validation("数据库名引号不匹配".to_string()));
                }
                
                let data_part = &after_insert[data_start_pos..].trim();
                if data_part.is_empty() {
                    return Err(AppError::Validation("INSERT 语句缺少数据行".to_string()));
                }
                Ok(data_part.to_string())
            } else {
                // 检查是否包含数据库名（没有引号）
                let parts: Vec<&str> = after_insert.split_whitespace().collect();
                if parts.len() >= 2 && !parts[0].contains('=') {
                    // 第一个部分可能是数据库名
                    Ok(parts[1..].join(" "))
                } else {
                    // 直接是数据行
                    Ok(after_insert.to_string())
                }
            }
        }
    }

    async fn get_databases(&self) -> Result<Vec<String>, AppError> {
        tracing::info!("[BE] InfluxDBV1Service::get_databases called");
        let query = "SHOW DATABASES";
        tracing::info!("[BE] Executing query: '{}'", query);
        
        let result = self.query(query).await?;
        tracing::info!("[BE] Query result: {:?} series", result.series.len());
        
        let mut databases = Vec::new();
        for (series_index, series) in result.series.iter().enumerate() {
            tracing::info!("[BE] Processing series {}: name='{}', columns={:?}", 
                          series_index, series.name, series.columns);
            
            for (row_index, row) in series.values.iter().enumerate() {
                tracing::info!("[BE] Processing row {}: {:?}", row_index, row);
                
                // 修复：数据库名称在索引 0，不是索引 1
                if let Some(db_name) = row.get(0) {
                    if let Some(name) = db_name.as_str() {
                        tracing::info!("[BE] Found database: {}", name);
                        databases.push(name.to_string());
                    } else {
                        tracing::warn!("[BE] Database name is not a string: {:?}", db_name);
                    }
                } else {
                    tracing::warn!("[BE] No database name found at index 0 in row: {:?}", row);
                }
            }
        }
        
        tracing::info!("[BE] get_databases completed, found {} databases: {:?}", databases.len(), databases);
        Ok(databases)
    }

    async fn get_database_info(&self, database: &str) -> Result<DatabaseInfo, AppError> {
        // 获取保留策略
        let rp_query = format!("SHOW RETENTION POLICIES ON {database}");
        let rp_result = self.query(&rp_query).await?;
        
        let mut retention_policies = Vec::new();
        for series in rp_result.series {
            for row in series.values {
                if row.len() >= 4 {
                    retention_policies.push(crate::models::RetentionPolicy {
                        name: row[0].as_str().unwrap_or("").to_string(),
                        duration: row[1].as_str().unwrap_or("").to_string(),
                        replication: row[2].as_u64().unwrap_or(1) as u32,
                        default: row[3].as_bool().unwrap_or(false),
                    });
                }
            }
        }
        
        // 获取测量值
        let measurements = self.get_measurements(database).await?;
        let measurements_info = measurements.into_iter().map(|name| {
            crate::models::Measurement {
                name,
                tag_keys: Vec::new(), // 简化实现
                field_keys: Vec::new(), // 简化实现
                series_count: 0, // 简化实现
            }
        }).collect();
        
        Ok(DatabaseInfo {
            name: database.to_string(),
            retention_policies,
            measurements: measurements_info,
            series_count: 0, // 简化实现
        })
    }

    async fn get_measurements(&self, database: &str) -> Result<Vec<String>, AppError> {
        let query = format!("SHOW MEASUREMENTS ON {database}");
        let result = self.query(&query).await?;
        
        let mut measurements = Vec::new();
        for series in result.series {
            for row in series.values {
                if let Some(measurement) = row.get(0) {
                    if let Some(name) = measurement.as_str() {
                        measurements.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(measurements)
    }

    fn parse_query_response(&self, response_text: &str) -> Result<Vec<Series>, AppError> {
        let json: Value = serde_json::from_str(response_text)
            .map_err(|e| AppError::Parse(format!("Failed to parse JSON: {}", e)))?;
        
        let mut series = Vec::new();
        
        if let Some(results) = json.get("results").and_then(|v| v.as_array()) {
            for result in results {
                if let Some(result_series) = result.get("series").and_then(|v| v.as_array()) {
                    for series_data in result_series {
                        if let Ok(series_item) = self.parse_series(series_data) {
                            series.push(series_item);
                        }
                    }
                }
            }
        }
        
        Ok(series)
    }
    
    fn parse_series(&self, series_data: &Value) -> Result<Series, AppError> {
        let name = series_data.get("name")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        
        let columns = series_data.get("columns")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect())
            .unwrap_or_default();
        
        let values = series_data.get("values")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.as_array())
                .map(|row| row.iter().cloned().collect())
                .collect())
            .unwrap_or_default();
        
        let tags = series_data.get("tags")
            .and_then(|v| v.as_object())
            .map(|obj| obj.iter()
                .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                .collect());
        
        Ok(Series {
            name,
            columns,
            values,
            tags,
        })
    }
}

/// InfluxDB v2.x 服务实现
pub struct InfluxDBV2Service {
    client: Client,
    config: InfluxDBV2Config,
    base_url: String,
}

impl InfluxDBV2Service {
    pub async fn new(config: InfluxDBV2Config) -> Result<Self, AppError> {
        tracing::info!(
            "Creating InfluxDB v2 service for host: {}:{}",
            config.host,
            config.port
        );
        let protocol = if config.use_ssl { "https" } else { "http" };
        let base_url = format!("{}://{}:{}", protocol, config.host, config.port);
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_millis(config.timeout))
            .build()
            .map_err(|e| AppError::Network(e.to_string()))?;
        
        Ok(Self {
            client,
            config,
            base_url,
        })
    }

    async fn ping(&self) -> Result<bool, AppError> {
        tracing::info!("Pinging v2 service at {}", self.base_url);
        let url = format!("{}/ping", self.base_url);
        let token = &self.config.token;
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Token {token}"))
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;

        if response.status().is_success() {
            Ok(true)
        } else {
            let status = response.status();
            let text = response.text().await.unwrap_or_else(|_| "N/A".to_string());
            let error_message = format!(
                "Connection failed with status: {}. Response: {}",
                status, text
            );
            tracing::warn!("{}", error_message);
            Err(AppError::Network(error_message))
        }
    }

    async fn query(&self, query: &str) -> Result<QueryResult, AppError> {
        // 使用配置中的默认bucket
        let default_bucket = "mybucket".to_string();
        let bucket = self.config.bucket.as_ref().unwrap_or(&default_bucket);
        self.query_with_database(query, bucket).await
    }

    async fn query_with_database(&self, query: &str, bucket: &str) -> Result<QueryResult, AppError> {
        tracing::info!("Executing v2 query (raw): '{}' with bucket: '{}'", query, bucket);
        let start = std::time::Instant::now();
        let org = &self.config.org;
        let token = &self.config.token;
        
        // 构建带有 org 参数的 URL
        let url = format!("{}/api/v2/query?org={org}", self.base_url);
        
        // InfluxDB 2.x 使用 Flux 查询语言
        let flux_query = self.convert_to_flux_with_bucket(query, bucket)?;
        tracing::info!("Executing v2 query (flux): '{}'", flux_query);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {token}"))
            .header("Content-Type", "application/vnd.flux")
            .body(flux_query)
            .send()
            .await
            .map_err(|e| AppError::Network(e.to_string()))?;
        
        let status = response.status();
        
        if status.is_success() {
            let response_text = response.text().await
                .map_err(|e| AppError::Network(e.to_string()))?;
            let series = self.parse_flux_response(&response_text)?;
            let execution_time = start.elapsed().as_millis() as u64;
            
            tracing::info!("Query executed successfully in {}ms", execution_time);
            Ok(QueryResult {
                series,
                execution_time,
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            let error_message = format!("HTTP {status}: {error_text}");
            tracing::error!("Query failed: {}", error_message);
            Err(AppError::Query(error_message))
        }
    }

    async fn get_databases(&self) -> Result<Vec<String>, AppError> {
        // InfluxDB 2.x 使用 buckets 而不是 databases
        let query = "buckets()";
        let result = self.query(query).await?;
        
        let mut buckets = Vec::new();
        for series in result.series {
            for row in series.values {
                if let Some(bucket_name) = row.get(0) {
                    if let Some(name) = bucket_name.as_str() {
                        buckets.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(buckets)
    }

    async fn get_database_info(&self, bucket: &str) -> Result<DatabaseInfo, AppError> {
        // InfluxDB 2.x 的 bucket 信息
        let measurements = self.get_measurements(bucket).await?;
        let measurements_info = measurements.into_iter().map(|name| {
            crate::models::Measurement {
                name,
                tag_keys: Vec::new(), // 简化实现
                field_keys: Vec::new(), // 简化实现
                series_count: 0, // 简化实现
            }
        }).collect();
        
        Ok(DatabaseInfo {
            name: bucket.to_string(),
            retention_policies: Vec::new(), // InfluxDB 2.x 使用 retention rules
            measurements: measurements_info,
            series_count: 0, // 简化实现
        })
    }

    async fn get_measurements(&self, bucket: &str) -> Result<Vec<String>, AppError> {
        let query = format!("import \"influxdata/influxdb/schema\"\nschema.measurements(bucket: \"{bucket}\")");
        let result = self.query(&query).await?;
        
        let mut measurements = Vec::new();
        for series in result.series {
            for row in series.values {
                if let Some(measurement) = row.get(0) {
                    if let Some(name) = measurement.as_str() {
                        measurements.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(measurements)
    }

    fn convert_to_flux(&self, query: &str) -> Result<String, AppError> {
        // 使用配置中的默认bucket
        let default_bucket = "mybucket".to_string();
        let bucket = self.config.bucket.as_ref().unwrap_or(&default_bucket);
        self.convert_to_flux_with_bucket(query, bucket)
    }

    fn convert_to_flux_with_bucket(&self, query: &str, bucket: &str) -> Result<String, AppError> {
        let query_upper = query.to_uppercase();
        
        if query_upper.starts_with("SHOW DATABASES") {
            Ok("buckets() |> rename(columns: {name: \"name\"}) |> keep(columns: [\"name\"])".to_string())
        } else if query_upper.starts_with("SHOW MEASUREMENTS") {
            Ok(format!("import \"influxdata/influxdb/schema\"\nschema.measurements(bucket: \"{bucket}\")"))
        } else {
            // 简化版本：假设已经是 Flux 查询
            Ok(query.to_string())
        }
    }
    
    fn parse_flux_response(&self, response_text: &str) -> Result<Vec<Series>, AppError> {
        // Flux 响应是 CSV 格式
        let mut series = Vec::new();
        let lines: Vec<&str> = response_text.lines().collect();
        
        if lines.is_empty() {
            return Ok(series);
        }
        
        // 解析 CSV 格式的 Flux 响应
        let mut current_series = Series {
            name: "flux_result".to_string(),
            columns: Vec::new(),
            values: Vec::new(),
            tags: None,
        };
        
        for (i, line) in lines.iter().enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            
            let values: Vec<&str> = line.split(',').collect();
            if i == 0 {
                // 第一行是列名
                current_series.columns = values.iter().map(|s| s.to_string()).collect();
            } else {
                // 数据行
                let row_values: Vec<Value> = values.iter()
                    .map(|s| Value::String(s.to_string()))
                    .collect();
                current_series.values.push(row_values);
            }
        }
        
        if !current_series.columns.is_empty() {
            series.push(current_series);
        }
        
        Ok(series)
    }
}

/// 工厂函数：根据连接配置创建对应的服务
pub async fn create_influxdb_service(profile: &ConnectionProfile) -> Result<InfluxDBService, AppError> {
    match profile.version {
        InfluxDBVersion::V1 => {
            let config = profile.get_v1_config()
                .map_err(|e| AppError::Config(format!("Invalid v1 config: {}", e)))?;
            let service = InfluxDBV1Service::new(config).await?;
            Ok(InfluxDBService::V1(service))
        }
        InfluxDBVersion::V2 => {
            let config = profile.get_v2_config()
                .map_err(|e| AppError::Config(format!("Invalid v2 config: {}", e)))?;
            let service = InfluxDBV2Service::new(config).await?;
            Ok(InfluxDBService::V2(service))
        }
        InfluxDBVersion::V3 => {
            // 简化实现：对于 v3.x，暂时使用 v2.x 的服务
            let config = profile.get_v2_config()
                .map_err(|e| AppError::Config(format!("Invalid v3 config: {}", e)))?;
            let service = InfluxDBV2Service::new(config).await?;
            Ok(InfluxDBService::V2(service))
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_insert_query() {
        let service = InfluxDBV1Service {
            client: reqwest::Client::new(),
            config: InfluxDBV1Config {
                host: "localhost".to_string(),
                port: 8086,
                database: "test".to_string(),
                username: None,
                password: None,
                use_ssl: false,
                timeout: 5000,
            },
            base_url: "http://localhost:8086".to_string(),
        };

        // 测试用例 - 支持多种 INSERT 语法
        let test_cases = vec![
            (
                r#"INSERT INTO "testdb" cpu,host=server01 value=0.64"#,
                Ok::<String, AppError>("cpu,host=server01 value=0.64".to_string()),
            ),
            (
                r#"INSERT INTO testdb memory,host=server01 value=0.32"#,
                Ok::<String, AppError>("memory,host=server01 value=0.32".to_string()),
            ),
            (
                r#"INSERT INTO "my-db" temperature,location=room1 value=25.5"#,
                Ok::<String, AppError>("temperature,location=room1 value=25.5".to_string()),
            ),
            // 新增：支持不带 INTO 的语法
            (
                r#"INSERT "testdb" cpu,host=server01 value=0.64"#,
                Ok::<String, AppError>("cpu,host=server01 value=0.64".to_string()),
            ),
            (
                r#"INSERT testdb memory,host=server01 value=0.32"#,
                Ok::<String, AppError>("memory,host=server01 value=0.32".to_string()),
            ),
            // 新增：支持直接插入到默认数据库
            (
                r#"INSERT cpu,host=server01 value=0.64"#,
                Ok::<String, AppError>("cpu,host=server01 value=0.64".to_string()),
            ),
        ];

        for (input, expected) in test_cases {
            let result = service.parse_insert_query(input);
            match (result.clone(), expected.clone()) {
                (Ok(actual), Ok(expected)) => {
                    assert_eq!(actual, expected, "Failed for input: {}", input);
                }
                (Err(actual), Err(expected)) => {
                    assert_eq!(actual.to_string(), expected.to_string(), "Failed for input: {}", input);
                }
                _ => {
                    panic!("Mismatch for input: {}, got {:?}, expected {:?}", input, result, expected);
                }
            }
        }
    }
} 