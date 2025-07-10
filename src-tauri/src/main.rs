// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod error;
mod influxdb;
mod models;
mod commands;

use std::sync::Mutex;
use std::collections::HashMap;
use crate::influxdb::InfluxDbClient;
use commands::{
    test_connection,
    connect_to_database,
    disconnect_from_database,
    get_databases,
    get_database_info,
    execute_query,
    create_database,
    drop_database,
    get_measurements,
    get_app_version
};

// 全局连接管理器
type ConnectionMap = Mutex<HashMap<String, InfluxDbClient>>;

fn main() {
    // 初始化日志系统
    tracing_subscriber::fmt::init();
    
    // 创建全局连接管理器
    let connections: ConnectionMap = Mutex::new(HashMap::new());
    
    tauri::Builder::default()
        .manage(connections)
        .invoke_handler(tauri::generate_handler![
            test_connection,
            connect_to_database,
            disconnect_from_database,
            get_databases,
            get_database_info,
            execute_query,
            create_database,
            drop_database,
            get_measurements,
            get_app_version
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 