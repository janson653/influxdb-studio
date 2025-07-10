use serde::{Deserialize, Serialize};
use std::fmt;

/// 应用程序错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppError {
    /// 网络连接错误
    NetworkError(String),
    /// 数据库连接错误
    DatabaseError(String),
    /// 查询执行错误
    QueryError(String),
    /// 配置错误
    ConfigError(String),
    /// 序列化/反序列化错误
    SerializationError(String),
    /// 文件系统错误
    FileSystemError(String),
    /// 权限错误
    PermissionError(String),
    /// 超时错误
    TimeoutError(String),
    /// 通用错误
    GenericError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NetworkError(msg) => write!(f, "网络错误: {}", msg),
            AppError::DatabaseError(msg) => write!(f, "数据库错误: {}", msg),
            AppError::QueryError(msg) => write!(f, "查询错误: {}", msg),
            AppError::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            AppError::SerializationError(msg) => write!(f, "序列化错误: {}", msg),
            AppError::FileSystemError(msg) => write!(f, "文件系统错误: {}", msg),
            AppError::PermissionError(msg) => write!(f, "权限错误: {}", msg),
            AppError::TimeoutError(msg) => write!(f, "超时错误: {}", msg),
            AppError::GenericError(msg) => write!(f, "错误: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            AppError::TimeoutError(err.to_string())
        } else if err.is_connect() {
            AppError::NetworkError(err.to_string())
        } else {
            AppError::GenericError(err.to_string())
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::SerializationError(err.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::FileSystemError(err.to_string())
    }
}

/// 应用程序结果类型
pub type AppResult<T> = Result<T, AppError>; 