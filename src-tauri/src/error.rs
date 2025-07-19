use std::fmt;

/// 应用程序错误类型
#[derive(Debug)]
pub enum AppError {
    /// 网络连接错误
    Network(String),
    /// 数据库连接错误
    Database(String),
    /// 查询执行错误
    Query(String),
    /// 配置错误
    Config(String),
    /// 序列化/反序列化错误
    Serialization(String),
    /// 文件系统错误
    FileSystem(String),
    /// 权限错误
    Permission(String),
    /// 超时错误
    Timeout(String),
    /// 通用错误
    Generic(String),
    /// 解析错误
    Parse(String),
    /// 认证错误
    Auth(String),
    /// 未找到错误
    NotFound(String),
    /// 验证错误
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Query(msg) => write!(f, "Query error: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration error: {}", msg),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
            AppError::Database(msg) => write!(f, "Database error: {}", msg),
            AppError::Auth(msg) => write!(f, "Authentication error: {}", msg),
            AppError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation error: {}", msg),
            AppError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            AppError::FileSystem(msg) => write!(f, "File system error: {}", msg),
            AppError::Permission(msg) => write!(f, "Permission error: {}", msg),
            AppError::Timeout(msg) => write!(f, "Timeout error: {}", msg),
            AppError::Generic(msg) => write!(f, "Generic error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::Timeout(err.to_string())
        } else if err.is_connect() {
            AppError::Network(err.to_string())
        } else {
            AppError::Generic(err.to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystem(err.to_string())
    }
} 