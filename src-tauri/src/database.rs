use crate::models::{ApiResponse, DatabaseInfo, RetentionPolicy};
use crate::error::AppError;
use crate::influxdb_trait::InfluxDB;

/// List all databases from the connected InfluxDB instance
pub async fn list_databases<T: InfluxDB>(service: &T) -> Result<ApiResponse<Vec<String>>, String> {
    tracing::info!("[BE] database::list_databases called");
    
    match service.get_databases().await {
        Ok(databases) => {
            tracing::info!("[BE] database::list_databases succeeded, found {} databases", databases.len());
            Ok(ApiResponse {
                success: true,
                data: Some(databases),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] database::list_databases failed with error: {}", e);
            Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        }
    }
}

/// Get detailed information about a specific database
pub async fn get_database_info<T: InfluxDB>(
    service: &T,
    database: &str
) -> Result<ApiResponse<DatabaseInfo>, String> {
    tracing::info!("[BE] database::get_database_info called for database: {}", database);
    
    match service.get_database_info(database).await {
        Ok(info) => {
            tracing::info!("[BE] database::get_database_info succeeded for database: {}", database);
            Ok(ApiResponse {
                success: true,
                data: Some(info),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] database::get_database_info failed for database {}: {}", database, e);
            Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        }
    }
}

/// Create a new database with optional retention policy
pub async fn create_database<T: InfluxDB>(
    service: &T,
    database: &str,
    retention_policy: Option<RetentionPolicy>,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] database::create_database called for database: {}", database);
    
    // Validate database name
    if database.is_empty() {
        return Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some("Database name cannot be empty".to_string()),
        });
    }
    
    // Build the query based on whether a retention policy is provided
    let query = match retention_policy {
        Some(rp) => {
            format!(
                "CREATE DATABASE \"{}\" WITH DURATION {} REPLICATION {} NAME \"{}\"",
                database, rp.duration, rp.replication, rp.name
            )
        },
        None => {
            format!("CREATE DATABASE \"{}\"", database)
        }
    };
    
    // Execute the query
    match service.query(&query).await {
        Ok(_) => {
            tracing::info!("[BE] database::create_database succeeded for database: {}", database);
            Ok(ApiResponse {
                success: true,
                data: Some(true),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] database::create_database failed for database {}: {}", database, e);
            Ok(ApiResponse {
                success: false,
                data: Some(false),
                error: Some(e.to_string()),
            })
        }
    }
}

/// Delete an existing database
pub async fn delete_database<T: InfluxDB>(
    service: &T,
    database: &str,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] database::delete_database called for database: {}", database);
    
    // Validate database name
    if database.is_empty() {
        return Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some("Database name cannot be empty".to_string()),
        });
    }
    
    // Build the query
    let query = format!("DROP DATABASE \"{}\"", database);
    
    // Execute the query
    match service.query(&query).await {
        Ok(_) => {
            tracing::info!("[BE] database::delete_database succeeded for database: {}", database);
            Ok(ApiResponse {
                success: true,
                data: Some(true),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] database::delete_database failed for database {}: {}", database, e);
            Ok(ApiResponse {
                success: false,
                data: Some(false),
                error: Some(e.to_string()),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{DatabaseInfo, QueryResult};
    use async_trait::async_trait;

    struct MockInfluxDB {
        databases: Vec<String>,
        should_fail: bool,
    }

    #[async_trait]
    impl InfluxDB for MockInfluxDB {
        async fn get_databases(&self) -> Result<Vec<String>, AppError> {
            if self.should_fail {
                Err(AppError::Network("Test error".to_string()))
            } else {
                Ok(self.databases.clone())
            }
        }

        async fn get_database_info(&self, _db_name: &str) -> Result<DatabaseInfo, AppError> {
            unimplemented!()
        }

        async fn query(&self, _query: &str) -> Result<Vec<QueryResult>, AppError> {
            unimplemented!()
        }
    }

    #[tokio::test]
    async fn test_list_databases_success() {
        let mock_service = MockInfluxDB {
            databases: vec!["db1".to_string(), "db2".to_string()],
            should_fail: false,
        };

        let result = list_databases(&mock_service).await.unwrap();
        assert!(result.success);
        assert_eq!(result.data, Some(vec!["db1".to_string(), "db2".to_string()]));
        assert!(result.error.is_none());
    }

    #[tokio::test]
    async fn test_list_databases_error() {
        let mock_service = MockInfluxDB {
            databases: vec![],
            should_fail: true,
        };

        let result = list_databases(&mock_service).await.unwrap();
        assert!(!result.success);
        assert!(result.data.is_none());
        assert_eq!(result.error, Some("Test error".to_string()));
    }
}