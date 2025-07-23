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
    tracing::info!("[BE] connect_to_database called for profile: {} (id: {})", profile.name, connection_id);
    
    // 创建服务
    let service = match create_influxdb_service(&profile).await {
        Ok(service) => {
            tracing::info!("[BE] Successfully created service for profile: {}", profile.name);
            service
        },
        Err(e) => {
            tracing::error!("[BE] Failed to create service for profile {}: {}", profile.name, e);
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
        tracing::info!("[BE] Storing connection in map with id: {}", connection_id);
        conn_map.insert(connection_id.clone(), Arc::new(service));
        tracing::info!("[BE] Current connections after insert: {:?}", conn_map.keys().collect::<Vec<_>>());
    }
    
    tracing::info!("[BE] connect_to_database succeeded, returning connection_id: {}", connection_id);
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
    tracing::info!("[BE] get_databases called with connection_id: {}", connection_id);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        tracing::info!("[BE] Current connections in map: {:?}", conn_map.keys().collect::<Vec<_>>());
        
        match conn_map.get(&connection_id) {
            Some(service) => {
                tracing::info!("[BE] Found service for connection_id: {}", connection_id);
                service.clone()
            },
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the database module to handle the operation
    crate::database::list_databases(&*service).await
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
    
    // Use the database module to handle the operation
    crate::database::get_database_info(&*service, &database).await
}

/// 执行查询
#[tauri::command]
pub async fn execute_query(
    connection_id: String,
    database: String,
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
    match service.query_with_database(&query, &database).await {
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
    retention_policy: Option<crate::models::RetentionPolicy>,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] create_database called with connection_id: {}, database: {}", connection_id, database);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the database module to handle the operation
    crate::database::create_database(&*service, &database, retention_policy).await
}

/// 删除数据库
#[tauri::command]
pub async fn drop_database(
    connection_id: String,
    database: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] drop_database called with connection_id: {}, database: {}", connection_id, database);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the database module to handle the operation
    crate::database::delete_database(&*service, &database).await
}

/// 获取测量值列表
#[tauri::command]
pub async fn get_measurements(
    connection_id: String,
    database: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<Vec<String>>, String> {
    tracing::info!("[BE] get_measurements called with connection_id: {}, database: {}", connection_id, database);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        tracing::info!("[BE] Current connections in map: {:?}", conn_map.keys().collect::<Vec<_>>());
        
        match conn_map.get(&connection_id) {
            Some(service) => {
                tracing::info!("[BE] Found service for connection_id: {}", connection_id);
                service.clone()
            },
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the measurement module to handle the operation
    crate::measurement::list_measurements(&*service, &database).await
}

/// 创建测量
#[tauri::command]
pub async fn create_measurement(
    connection_id: String,
    database: String,
    measurement: String,
    fields: Vec<(String, String, serde_json::Value)>, // (name, type, value)
    tags: Option<Vec<(String, String)>>, // (name, value)
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] create_measurement called with connection_id: {}, database: {}, measurement: {}", 
                  connection_id, database, measurement);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the measurement module to handle the operation
    crate::measurement::create_measurement(&*service, &database, &measurement, fields, tags).await
}

/// 获取测量信息
#[tauri::command]
pub async fn get_measurement_info(
    connection_id: String,
    database: String,
    measurement: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<crate::models::Measurement>, String> {
    tracing::info!("[BE] get_measurement_info called with connection_id: {}, database: {}, measurement: {}", 
                  connection_id, database, measurement);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the measurement module to handle the operation
    crate::measurement::get_measurement_info(&*service, &database, &measurement).await
}

/// 删除测量
#[tauri::command]
pub async fn delete_measurement(
    connection_id: String,
    database: String,
    measurement: String,
    connections: State<'_, ConnectionMap>,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] delete_measurement called with connection_id: {}, database: {}, measurement: {}", 
                  connection_id, database, measurement);
    
    // 获取服务引用
    let service = {
        let conn_map = connections.lock().unwrap();
        match conn_map.get(&connection_id) {
            Some(service) => service.clone(),
            None => {
                tracing::error!("[BE] Connection not found for connection_id: {}", connection_id);
                return Ok(ApiResponse {
                    success: false,
                    data: None,
                    error: Some("Connection not found".to_string()),
                });
            }
        }
    };
    
    // Use the measurement module to handle the operation
    crate::measurement::delete_measurement(&*service, &database, &measurement).await
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