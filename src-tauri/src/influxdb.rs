use crate::models::{ConnectionConfig, QueryResult, DatabaseInfo, Series};
use crate::error::AppError;
use reqwest::Client;
use serde_json::Value;

/// InfluxDB 客户端
#[derive(Clone)]
pub struct InfluxDbClient {
    client: Client,
    config: ConnectionConfig,
    base_url: String,
    username: Option<String>,
    password: Option<String>,
    // InfluxDB 2.x 支持
    token: Option<String>,
    org: Option<String>,
    is_v2: bool,
}

impl InfluxDbClient {
    /// 创建新的 InfluxDB 客户端
    pub async fn new(config: ConnectionConfig) -> Result<Self, AppError> {
        let protocol = if config.use_ssl { "https" } else { "http" };
        let base_url = format!("{}://{}:{}", protocol, config.host, config.port);
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        
        // 检测是否为 InfluxDB 2.x (有 token 字段)
        let is_v2 = config.token.is_some();
        
        Ok(Self {
            client,
            config: config.clone(),
            base_url,
            username: config.username,
            password: config.password,
            token: config.token,
            org: config.org,
            is_v2,
        })
    }
    
    /// 测试连接
    pub async fn ping(&self) -> Result<bool, AppError> {
        let url = format!("{}/ping", self.base_url);
        let response = self.client.get(&url).send().await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        Ok(response.status().is_success())
    }
    
    /// 获取数据库列表
    pub async fn get_databases(&self) -> Result<Vec<String>, AppError> {
        let query = "SHOW DATABASES";
        let result = self.query("", query).await?;
        
        let mut databases = Vec::new();
        for series in result.series {
            for row in series.values {
                if let Some(db_name) = row.get(1) {
                    if let Some(name) = db_name.as_str() {
                        databases.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(databases)
    }
    
    /// 获取数据库信息
    pub async fn get_database_info(&self, database: &str) -> Result<DatabaseInfo, AppError> {
        // 获取保留策略
        let rp_query = format!("SHOW RETENTION POLICIES ON {}", database);
        let rp_result = self.query(database, &rp_query).await?;
        
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
    
    /// 执行查询
    pub async fn query(&self, database: &str, query: &str) -> Result<QueryResult, AppError> {
        let start = std::time::Instant::now();
        
        // 添加调试日志
        tracing::info!("Executing query: {} on database: {}", query, database);
        tracing::info!("InfluxDB version: {}", if self.is_v2 { "2.x" } else { "1.x" });
        
        if self.is_v2 {
            self.query_v2(database, query, start).await
        } else {
            self.query_v1(database, query, start).await
        }
    }
    
    /// InfluxDB 1.x 查询
    async fn query_v1(&self, database: &str, query: &str, start: std::time::Instant) -> Result<QueryResult, AppError> {
        let url = format!("{}/query", self.base_url);
        let mut params = vec![
            ("db", database),
            ("q", query),
        ];
        
        tracing::info!("Username: {:?}, Password: {:?}", self.username, self.password.as_ref().map(|_| "***"));
        
        if let Some(username) = &self.username {
            params.push(("u", username));
        }
        if let Some(password) = &self.password {
            params.push(("p", password));
        }
        
        let response = self.client
            .post(&url)
            .query(&params)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        
        let status = response.status();
        
        if status.is_success() {
            let response_text = response.text().await
                .map_err(|e| AppError::NetworkError(e.to_string()))?;
            let series = self.parse_query_response_v1(&response_text)?;
            let execution_time = start.elapsed().as_millis() as u64;
            
            Ok(QueryResult {
                series,
                execution_time,
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(AppError::QueryError(format!("HTTP {}: {}", status, error_text)))
        }
    }
    
    /// InfluxDB 2.x 查询
    async fn query_v2(&self, _database: &str, query: &str, start: std::time::Instant) -> Result<QueryResult, AppError> {
        let org = self.org.as_ref().ok_or_else(|| AppError::ConfigError("InfluxDB 2.x requires org parameter".to_string()))?;
        let token = self.token.as_ref().ok_or_else(|| AppError::ConfigError("InfluxDB 2.x requires token parameter".to_string()))?;
        
        // 构建带有 org 参数的 URL
        let url = format!("{}/api/v2/query?org={}", self.base_url, org);
        
        tracing::info!("Org: {}, Token: {}", org, "***");
        tracing::info!("Request URL: {}", url);
        
        // InfluxDB 2.x 使用 Flux 查询语言
        let flux_query = if query.to_uppercase().starts_with("SHOW DATABASES") {
            // 转换 SHOW DATABASES 为 Flux 查询
            format!("buckets() |> rename(columns: {{name: \"name\"}}) |> keep(columns: [\"name\"])")
        } else if query.to_uppercase().starts_with("SHOW MEASUREMENTS") {
            // 转换 SHOW MEASUREMENTS 为 Flux 查询
            let default_bucket = "mybucket".to_string();
            let bucket = self.config.bucket.as_ref().unwrap_or(&default_bucket);
            format!("import \"influxdata/influxdb/schema\"\nschema.measurements(bucket: \"{}\")", bucket)
        } else {
            // 尝试将 SQL 转换为 Flux（简化版本）
            self.convert_sql_to_flux(query)?
        };
        
        tracing::info!("Flux query: {}", flux_query);
        
        // 使用 Content-Type: application/vnd.flux 并直接发送 Flux 查询
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", token))
            .header("Content-Type", "application/vnd.flux")
            .header("Accept", "application/csv")
            .body(flux_query)
            .send()
            .await
            .map_err(|e| AppError::NetworkError(e.to_string()))?;
        
        let status = response.status();
        
        if status.is_success() {
            let response_text = response.text().await
                .map_err(|e| AppError::NetworkError(e.to_string()))?;
            tracing::info!("Response: {}", response_text);
            let series = self.parse_query_response_v2(&response_text)?;
            let execution_time = start.elapsed().as_millis() as u64;
            
            Ok(QueryResult {
                series,
                execution_time,
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            tracing::error!("Query failed: HTTP {}: {}", status, error_text);
            Err(AppError::QueryError(format!("HTTP {}: {}", status, error_text)))
        }
    }
    
    /// 简单的 SQL 到 Flux 转换
    fn convert_sql_to_flux(&self, sql: &str) -> Result<String, AppError> {
        let default_bucket = "mybucket".to_string();
        let bucket = self.config.bucket.as_ref().unwrap_or(&default_bucket);
        
        // 简化版本：只处理基本的 SELECT 语句
        if sql.to_uppercase().contains("SELECT") && sql.to_uppercase().contains("FROM") {
            // 提取表名
            let parts: Vec<&str> = sql.split_whitespace().collect();
            if let Some(from_idx) = parts.iter().position(|&x| x.to_uppercase() == "FROM") {
                if let Some(table) = parts.get(from_idx + 1) {
                    let measurement = table.trim_matches(|c| c == '"' || c == '`');
                    return Ok(format!(
                        "from(bucket: \"{}\")\n  |> range(start: -1h)\n  |> filter(fn: (r) => r._measurement == \"{}\")\n  |> limit(n: 10)",
                        bucket, measurement
                    ));
                }
            }
        }
        
        // 默认查询
        Ok(format!(
            "from(bucket: \"{}\")\n  |> range(start: -1h)\n  |> limit(n: 10)",
            bucket
        ))
    }
    
    /// 创建数据库
    pub async fn create_database(&self, database: &str) -> Result<bool, AppError> {
        let query = format!("CREATE DATABASE \"{}\"", database);
        let _result = self.query("", &query).await?;
        Ok(true)
    }
    
    /// 删除数据库
    pub async fn drop_database(&self, database: &str) -> Result<bool, AppError> {
        let query = format!("DROP DATABASE \"{}\"", database);
        let _result = self.query("", &query).await?;
        Ok(true)
    }
    
    /// 获取测量值列表
    pub async fn get_measurements(&self, database: &str) -> Result<Vec<String>, AppError> {
        let query = format!("SHOW MEASUREMENTS ON {}", database);
        let result = self.query(database, &query).await?;
        
        let mut measurements = Vec::new();
        for series in result.series {
            for row in series.values {
                if let Some(measurement_name) = row.get(0) {
                    if let Some(name) = measurement_name.as_str() {
                        measurements.push(name.to_string());
                    }
                }
            }
        }
        
        Ok(measurements)
    }
    
    /// 获取标签键
    pub async fn get_tag_keys(&self, database: &str, measurement: &str) -> Result<Vec<String>, AppError> {
        let query = format!("SHOW TAG KEYS FROM \"{}\"", measurement);
        let result = self.query(database, &query).await?;
        
        let mut tag_keys = Vec::new();
        for series in result.series {
            for row in series.values {
                if let Some(tag_key) = row.get(1) {
                    if let Some(key) = tag_key.as_str() {
                        tag_keys.push(key.to_string());
                    }
                }
            }
        }
        
        Ok(tag_keys)
    }
    
    /// 获取字段键
    pub async fn get_field_keys(&self, database: &str, measurement: &str) -> Result<Vec<(String, String)>, AppError> {
        let query = format!("SHOW FIELD KEYS FROM \"{}\"", measurement);
        let result = self.query(database, &query).await?;
        
        let mut field_keys = Vec::new();
        for series in result.series {
            for row in series.values {
                if row.len() >= 2 {
                    let field_name = row[0].as_str().unwrap_or("").to_string();
                    let field_type = row[1].as_str().unwrap_or("").to_string();
                    field_keys.push((field_name, field_type));
                }
            }
        }
        
        Ok(field_keys)
    }
    
    /// 解析查询响应 (InfluxDB 1.x)
    fn parse_query_response_v1(&self, response_text: &str) -> Result<Vec<Series>, AppError> {
        let json: Value = serde_json::from_str(response_text)
            .map_err(|e| AppError::SerializationError(e.to_string()))?;
        
        if let Some(results) = json.get("results").and_then(|r| r.as_array()) {
            if let Some(first_result) = results.first() {
                if let Some(series) = first_result.get("series").and_then(|s| s.as_array()) {
                    let mut result_series = Vec::new();
                    
                    for series_obj in series {
                        if let (Some(name), Some(columns), Some(values)) = (
                            series_obj.get("name").and_then(|n| n.as_str()),
                            series_obj.get("columns").and_then(|c| c.as_array()),
                            series_obj.get("values").and_then(|v| v.as_array()),
                        ) {
                            let columns: Vec<String> = columns
                                .iter()
                                .filter_map(|c| c.as_str().map(|s| s.to_string()))
                                .collect();
                            
                            let values: Vec<Vec<serde_json::Value>> = values
                                .iter()
                                .filter_map(|v| v.as_array().map(|arr| arr.clone()))
                                .collect();
                            
                            result_series.push(Series {
                                name: name.to_string(),
                                columns,
                                values,
                                tags: None, // 简化实现
                            });
                        }
                    }
                    
                    return Ok(result_series);
                }
            }
        }
        
        Ok(Vec::new())
    }
    
    /// 解析查询响应 (InfluxDB 2.x)
    fn parse_query_response_v2(&self, response_text: &str) -> Result<Vec<Series>, AppError> {
        // InfluxDB 2.x 返回 CSV 格式的数据
        let mut series = Vec::new();
        let lines: Vec<&str> = response_text.lines().collect();
        
        if lines.is_empty() {
            return Ok(series);
        }
        
        // 跳过注释行，找到表头
        let mut header_line = None;
        let mut data_start = 0;
        
        for (i, line) in lines.iter().enumerate() {
            if !line.starts_with('#') && !line.trim().is_empty() {
                header_line = Some(line);
                data_start = i + 1;
                break;
            }
        }
        
        if let Some(header) = header_line {
            let columns: Vec<String> = header.split(',').map(|s| s.trim().to_string()).collect();
            let mut values = Vec::new();
            
            // 解析数据行
            for line in lines.iter().skip(data_start) {
                if !line.starts_with('#') && !line.trim().is_empty() {
                    let row: Vec<serde_json::Value> = line
                        .split(',')
                        .map(|s| {
                            let trimmed = s.trim();
                            // 尝试解析为数字，否则作为字符串
                            if let Ok(num) = trimmed.parse::<f64>() {
                                serde_json::Value::Number(serde_json::Number::from_f64(num).unwrap_or(serde_json::Number::from(0)))
                            } else {
                                serde_json::Value::String(trimmed.to_string())
                            }
                        })
                        .collect();
                    values.push(row);
                }
            }
            
            if !values.is_empty() {
                series.push(Series {
                    name: "result".to_string(),
                    columns,
                    values,
                    tags: None,
                });
            }
        }
        
        Ok(series)
    }
} 