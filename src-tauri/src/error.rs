use serde::{Deserialize, Serialize};
use std::fmt;

/// 应用程序错误类型
#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Network(msg) => write!(f, "网络错误: {msg}"),
            AppError::Database(msg) => write!(f, "数据库错误: {msg}"),
            AppError::Query(msg) => write!(f, "查询错误: {msg}"),
            AppError::Config(msg) => write!(f, "配置错误: {msg}"),
            AppError::Serialization(msg) => write!(f, "序列化错误: {msg}"),
            AppError::FileSystem(msg) => write!(f, "文件系统错误: {msg}"),
            AppError::Permission(msg) => write!(f, "权限错误: {msg}"),
            AppError::Timeout(msg) => write!(f, "超时错误: {msg}"),
            AppError::Generic(msg) => write!(f, "错误: {msg}"),
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