use crate::models::{ConnectionProfile, ApiResponse, QueryResult, DatabaseInfo};
use crate::influxdb::{InfluxDBService, create_influxdb_service};

use std::collections::HashMap;
use std::sync::{Mutex, Arc};
use tauri::State;

// 连接映射类型
pub type ConnectionMap = Mutex<HashMap<String, Arc<InfluxDBService>>>;

/// 测试连接
#[tauri::command]
pub async fn test_connection(profile: ConnectionProfile) -> Result<ApiResponse<bool>, String> {
    tracing::info!("Testing connection to {}:{}", profile.config.get("host").unwrap_or(&serde_json::Value::Null), profile.config.get("port").unwrap_or(&serde_json::Value::Null));
    
    match create_influxdb_service(&profile).await {
        Ok(service) => {
            // 尝试 ping 服务器
            match service.ping().await {
                Ok(true) => {
                    tracing::info!("Connection test successful");
                    Ok(ApiResponse {
                        success: true,
                        data: Some(true),
                        error: None,
                    })
                },
                Ok(false) => {
                    tracing::warn!("Connection test failed - server not responding");
                    Ok(ApiResponse {
                        success: false,
                        data: Some(false),
                        error: Some("Server not responding".to_string()),
                    })
                },
                Err(e) => {
                    tracing::error!("Connection test failed: {}", e);
                    Ok(ApiResponse {
                        success: false,
                        data: Some(false),
                        error: Some(format!("Ping failed: {e}")),
                    })
                }
            }
        },
        Err(e) => {
            tracing::error!("Failed to create service: {}", e);
            Ok(ApiResponse {
                success: false,
                data: Some(false),
                error: Some(format!("Failed to create service: {e}")),
            })
        }
    }
}

/// 连接到数据库
#[tauri::command]
pub async fn connect_to_database(
    profile: ConnectionProfile,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<String>, String> {
    let connection_id = profile.id.clone();
    
    // 创建服务
    let service = match create_influxdb_service(&profile).await {
        Ok(service) => service,
        Err(e) => {
            return Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            });
        }
    };
    
    // 存储连接
    {
        let mut conn_map = connections.lock().unwrap();
        conn_map.insert(connection_id.clone(), Arc::new(service));
    }
    
    Ok(ApiResponse {
        success: true,
        data: Some(connection_id),
        error: None,
    })
}

/// 断开数据库连接
#[tauri::command]
pub async fn disconnect_from_database(
    connection_id: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    let mut conn_map = connections.lock().unwrap();
    
    match conn_map.remove(&connection_id) {
        Some(_) => Ok(ApiResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        None => Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some("Connection not found".to_string()),
        }),
    }
}

/// 获取数据库列表
#[tauri::command]
pub async fn get_databases(
    connection_id: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<Vec<String>>, String> {
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // 执行查询
    match service.get_databases().await {
        Ok(databases) => Ok(ApiResponse {
            success: true,
            data: Some(databases),
            error: None,
        }),
        Err(e) => Ok(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// 获取数据库信息
#[tauri::command]
pub async fn get_database_info(
    connection_id: String,
    database: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<DatabaseInfo>, String> {
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // 执行查询
    match service.get_database_info(&database).await {
        Ok(info) => Ok(ApiResponse {
            success: true,
            data: Some(info),
            error: None,
        }),
        Err(e) => Ok(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// 执行查询
#[tauri::command]
pub async fn execute_query(
    connection_id: String,
    query: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<QueryResult>, String> {
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // 执行查询
    match service.query(&query).await {
        Ok(result) => Ok(ApiResponse {
            success: true,
            data: Some(result),
            error: None,
        }),
        Err(e) => Ok(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// 创建数据库
#[tauri::command]
pub async fn create_database(
    connection_id: String,
    database: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // 执行创建数据库查询
    let query = format!("CREATE DATABASE \"{database}\"");
    match service.query(&query).await {
        Ok(_) => Ok(ApiResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some(e.to_string()),
        }),
    }
}

/// 删除数据库
#[tauri::command]
pub async fn drop_database(
    connection_id: String,
    database: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // 执行删除数据库查询
    let query = format!("DROP DATABASE \"{database}\"");
    match service.query(&query).await {
        Ok(_) => Ok(ApiResponse {
            success: true,
            data: Some(true),
            error: None,
        }),
        Err(e) => Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some(e.to_string()),
        }),
    }
}

/// 获取测量值列表
#[tauri::command]
pub async fn get_measurements(
    connection_id: String,
    database: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<Vec<String>>, String> {
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // 执行查询
    match service.get_measurements(&database).await {
        Ok(measurements) => Ok(ApiResponse {
            success: true,
            data: Some(measurements),
            error: None,
        }),
        Err(e) => Ok(ApiResponse {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

/// 获取应用版本
#[tauri::command]
pub async fn get_app_version() -> Result<ApiResponse<String>, String> {
    Ok(ApiResponse {
        success: true,
        data: Some(env!("CARGO_PKG_VERSION").to_string()),
        error: None,
    })
} 