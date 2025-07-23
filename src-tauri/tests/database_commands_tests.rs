use crate::commands::database_commands::*;
use crate::models::*;
use crate::error::AppError;
use mockall::predicate::*;
use mockall::*;

// Mock InfluxDB client
mock! {
    InfluxDBClient {}
    
    impl InfluxDBClient {
        pub async fn list_databases(&self) -> Result<Vec<String>, AppError>;
        pub async fn create_database(&self, name: &str, retention_policy: Option<&str>) -> Result<(), AppError>;
        pub async fn delete_database(&self, name: &str) -> Result<(), AppError>;
    }
}

#[tokio::test]
async fn test_list_databases_success() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_list_databases()
        .times(1)
        .returning(|| Ok(vec!["test_db".to_string(), "prod_db".to_string()]));

    let result = list_databases(&mock_client).await;
    
    assert!(result.is_ok());
    let databases = result.unwrap();
    assert_eq!(databases.len(), 2);
    assert_eq!(databases[0].name, "test_db");
    assert_eq!(databases[1].name, "prod_db");
}

#[tokio::test]
async fn test_list_databases_error() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_list_databases()
        .times(1)
        .returning(|| Err(AppError::DatabaseError("Connection failed".to_string())));

    let result = list_databases(&mock_client).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::DatabaseError(msg) => assert_eq!(msg, "Connection failed"),
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_database_success() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_create_database()
        .with(eq("new_db"), eq(None::<&str>))
        .times(1)
        .returning(|_, _| Ok(()));

    let request = CreateDatabaseRequest {
        name: "new_db".to_string(),
        retention_policy: None,
    };

    let result = create_database(&mock_client, request).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_database_with_retention_policy() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_create_database()
        .with(eq("new_db"), eq(Some("30d")))
        .times(1)
        .returning(|_, _| Ok(()));

    let request = CreateDatabaseRequest {
        name: "new_db".to_string(),
        retention_policy: Some("30d".to_string()),
    };

    let result = create_database(&mock_client, request).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_database_invalid_name() {
    let mock_client = MockInfluxDBClient::new();

    let request = CreateDatabaseRequest {
        name: "".to_string(),
        retention_policy: None,
    };

    let result = create_database(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Database name cannot be empty")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_database_special_characters() {
    let mock_client = MockInfluxDBClient::new();

    let request = CreateDatabaseRequest {
        name: "test@db".to_string(),
        retention_policy: None,
    };

    let result = create_database(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Database name contains invalid characters")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_delete_database_success() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_delete_database()
        .with(eq("test_db"))
        .times(1)
        .returning(|_| Ok(()));

    let request = DeleteDatabaseRequest {
        name: "test_db".to_string(),
        confirm: true,
    };

    let result = delete_database(&mock_client, request).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_database_without_confirmation() {
    let mock_client = MockInfluxDBClient::new();

    let request = DeleteDatabaseRequest {
        name: "test_db".to_string(),
        confirm: false,
    };

    let result = delete_database(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Confirmation required for database deletion")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_delete_database_error() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_delete_database()
        .with(eq("test_db"))
        .times(1)
        .returning(|_| Err(AppError::DatabaseError("Database not found".to_string())));

    let request = DeleteDatabaseRequest {
        name: "test_db".to_string(),
        confirm: true,
    };

    let result = delete_database(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::DatabaseError(msg) => assert_eq!(msg, "Database not found"),
        _ => panic!("Expected DatabaseError"),
    }
} 