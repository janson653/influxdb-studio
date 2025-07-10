use crate::models::{ConnectionConfig, QueryResult, DatabaseInfo, Series};
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
}

impl InfluxDbClient {
    /// 创建新的 InfluxDB 客户端
    pub async fn new(config: ConnectionConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let protocol = if config.use_ssl { "https" } else { "http" };
        let base_url = format!("{}://{}:{}", protocol, config.host, config.port);
        
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout))
            .build()?;
        
        Ok(Self {
            client,
            config: config.clone(),
            base_url,
            username: config.username,
            password: config.password,
        })
    }
    
    /// 测试连接
    pub async fn ping(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let url = format!("{}/ping", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
    
    /// 获取数据库列表
    pub async fn get_databases(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    pub async fn get_database_info(&self, database: &str) -> Result<DatabaseInfo, Box<dyn std::error::Error>> {
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
    pub async fn query(&self, database: &str, query: &str) -> Result<QueryResult, Box<dyn std::error::Error>> {
        let start = std::time::Instant::now();
        
        let url = format!("{}/query", self.base_url);
        let mut params = vec![
            ("db", database),
            ("q", query),
        ];
        
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
            .await?;
        
        let status = response.status();
        
        if status.is_success() {
            let response_text = response.text().await?;
            let series = self.parse_query_response(&response_text)?;
            let execution_time = start.elapsed().as_millis() as u64;
            
            Ok(QueryResult {
                series,
                execution_time,
            })
        } else {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(format!("HTTP {}: {}", status, error_text).into())
        }
    }
    
    /// 创建数据库
    pub async fn create_database(&self, database: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let query = format!("CREATE DATABASE \"{}\"", database);
        let _result = self.query("", &query).await?;
        Ok(true)
    }
    
    /// 删除数据库
    pub async fn drop_database(&self, database: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let query = format!("DROP DATABASE \"{}\"", database);
        let _result = self.query("", &query).await?;
        Ok(true)
    }
    
    /// 获取测量值列表
    pub async fn get_measurements(&self, database: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    pub async fn get_tag_keys(&self, database: &str, measurement: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
    pub async fn get_field_keys(&self, database: &str, measurement: &str) -> Result<Vec<(String, String)>, Box<dyn std::error::Error>> {
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
    
    /// 解析查询响应
    fn parse_query_response(&self, response_text: &str) -> Result<Vec<Series>, Box<dyn std::error::Error>> {
        let json: Value = serde_json::from_str(response_text)?;
        
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
} 