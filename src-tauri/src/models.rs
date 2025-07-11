use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 连接配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionConfig {
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