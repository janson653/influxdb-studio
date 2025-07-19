use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// InfluxDB 版本枚举
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InfluxDBVersion {
    #[serde(rename = "v1.x")]
    V1,
    #[serde(rename = "v2.x")]
    V2,
    #[serde(rename = "v3.x")]
    V3,
}

/// InfluxDB v1.x 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluxDBV1Config {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "useSsl")]
    pub use_ssl: bool,
    pub timeout: u64,
}

/// InfluxDB v2.x 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InfluxDBV2Config {
    pub host: String,
    pub port: u16,
    pub token: String,
    pub org: String,
    pub bucket: Option<String>,
    #[serde(rename = "useSsl")]
    pub use_ssl: bool,
    pub timeout: u64,
}

/// 统一的连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionProfile {
    pub id: String,
    pub name: String,
    pub version: InfluxDBVersion,
    pub config: serde_json::Value, // 动态配置，根据版本反序列化
    pub created_at: u64,
    pub updated_at: u64,
}

/// 连接状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionStatus {
    pub id: String,
    pub status: ConnectionStatusType,
    pub last_ping: Option<u64>,
    pub error: Option<String>,
    pub backend_connection_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatusType {
    Connected,
    Disconnected,
    Connecting,
    Error,
}

/// 兼容旧版本的连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegacyConnectionConfig {
    pub id: String,
    pub name: String,
    pub host: String,
    pub port: u16,
    pub database: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "useSsl")]
    pub use_ssl: bool,
    pub timeout: u64,
    // InfluxDB 2.x 支持
    pub token: Option<String>,
    pub org: Option<String>,
    pub bucket: Option<String>,
}

/// API 响应结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

/// 查询结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub series: Vec<Series>,
    pub execution_time: u64,
}

/// 数据系列
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Series {
    pub name: String,
    pub columns: Vec<String>,
    pub values: Vec<Vec<serde_json::Value>>,
    pub tags: Option<HashMap<String, String>>,
}

/// 数据库信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseInfo {
    pub name: String,
    pub retention_policies: Vec<RetentionPolicy>,
    pub measurements: Vec<Measurement>,
    pub series_count: u64,
}

/// 保留策略
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionPolicy {
    pub name: String,
    pub duration: String,
    pub replication: u32,
    pub default: bool,
}

/// 测量值信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measurement {
    pub name: String,
    pub tag_keys: Vec<String>,
    pub field_keys: Vec<FieldKey>,
    pub series_count: u64,
}

/// 字段键
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldKey {
    pub name: String,
    pub field_type: String,
}

/// 类型转换辅助函数
impl ConnectionProfile {
    /// 将配置反序列化为具体的配置类型
    pub fn get_v1_config(&self) -> Result<InfluxDBV1Config, serde_json::Error> {
        serde_json::from_value(self.config.clone())
    }

    pub fn get_v2_config(&self) -> Result<InfluxDBV2Config, serde_json::Error> {
        serde_json::from_value(self.config.clone())
    }

    /// 获取连接 URL
    pub fn get_connection_url(&self) -> Result<String, serde_json::Error> {
        match self.version {
            InfluxDBVersion::V1 => {
                let config = self.get_v1_config()?;
                let protocol = if config.use_ssl { "https" } else { "http" };
                Ok(format!("{}://{}:{}", protocol, config.host, config.port))
            }
            InfluxDBVersion::V2 => {
                let config = self.get_v2_config()?;
                let protocol = if config.use_ssl { "https" } else { "http" };
                Ok(format!("{}://{}:{}", protocol, config.host, config.port))
            }
            InfluxDBVersion::V3 => {
                // 对于 v3.x，使用 v2.x 的配置格式
                let config = self.get_v2_config()?;
                let protocol = if config.use_ssl { "https" } else { "http" };
                Ok(format!("{}://{}:{}", protocol, config.host, config.port))
            }
        }
    }
} 