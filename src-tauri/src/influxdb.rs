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
        tracing::info!("[BE] InfluxDBV1Service::query called with query: '{}'", query);
        let start = std::time::Instant::now();
        
        let url = format!("{}/query", self.base_url);
        tracing::info!("[BE] Making HTTP request to: {}", url);
        
        let mut request_builder = self.client
            .get(&url)
            .query(&[("q", query)]);
        
        // 添加数据库参数
        request_builder = request_builder.query(&[("db", &self.config.database)]);
        tracing::info!("[BE] Added database parameter: {}", self.config.database);
        
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
        tracing::info!("Executing v2 query (raw): '{}'", query);
        let start = std::time::Instant::now();
        let org = &self.config.org;
        let token = &self.config.token;
        
        // 构建带有 org 参数的 URL
        let url = format!("{}/api/v2/query?org={org}", self.base_url);
        
        // InfluxDB 2.x 使用 Flux 查询语言
        let flux_query = self.convert_to_flux(query)?;
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
        let query_upper = query.to_uppercase();
        
        if query_upper.starts_with("SHOW DATABASES") {
            Ok("buckets() |> rename(columns: {name: \"name\"}) |> keep(columns: [\"name\"])".to_string())
            } else if query_upper.starts_with("SHOW MEASUREMENTS") {
        let default_bucket = "mybucket".to_string();
        let bucket = self.config.bucket.as_ref().unwrap_or(&default_bucket);
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