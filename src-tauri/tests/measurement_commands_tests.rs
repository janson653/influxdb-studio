use crate::commands::measurement_commands::*;
use crate::models::*;
use crate::error::AppError;
use mockall::predicate::*;
use mockall::*;

// Mock InfluxDB client
mock! {
    InfluxDBClient {}
    
    impl InfluxDBClient {
        pub async fn list_measurements(&self, database: &str) -> Result<Vec<String>, AppError>;
        pub async fn create_measurement(&self, database: &str, name: &str, fields: &[Field], tags: &[Tag]) -> Result<(), AppError>;
        pub async fn delete_measurement(&self, database: &str, name: &str) -> Result<(), AppError>;
    }
}

#[tokio::test]
async fn test_list_measurements_success() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_list_measurements()
        .with(eq("test_db"))
        .times(1)
        .returning(|_| Ok(vec!["cpu".to_string(), "memory".to_string()]));

    let result = list_measurements(&mock_client, "test_db").await;
    
    assert!(result.is_ok());
    let measurements = result.unwrap();
    assert_eq!(measurements.len(), 2);
    assert_eq!(measurements[0].name, "cpu");
    assert_eq!(measurements[1].name, "memory");
}

#[tokio::test]
async fn test_list_measurements_error() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_list_measurements()
        .with(eq("test_db"))
        .times(1)
        .returning(|_| Err(AppError::DatabaseError("Database not found".to_string())));

    let result = list_measurements(&mock_client, "test_db").await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::DatabaseError(msg) => assert_eq!(msg, "Database not found"),
        _ => panic!("Expected DatabaseError"),
    }
}

#[tokio::test]
async fn test_create_measurement_success() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_create_measurement()
        .with(eq("test_db"), eq("new_measurement"), predicate(|fields: &[Field]| fields.len() == 2), predicate(|tags: &[Tag]| tags.len() == 1))
        .times(1)
        .returning(|_, _, _, _| Ok(()));

    let request = CreateMeasurementRequest {
        database: "test_db".to_string(),
        name: "new_measurement".to_string(),
        fields: vec![
            Field {
                name: "value".to_string(),
                field_type: FieldType::Float,
            },
            Field {
                name: "status".to_string(),
                field_type: FieldType::String,
            },
        ],
        tags: vec![
            Tag {
                name: "host".to_string(),
                value: "server1".to_string(),
            },
        ],
    };

    let result = create_measurement(&mock_client, request).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_measurement_invalid_name() {
    let mock_client = MockInfluxDBClient::new();

    let request = CreateMeasurementRequest {
        database: "test_db".to_string(),
        name: "".to_string(),
        fields: vec![],
        tags: vec![],
    };

    let result = create_measurement(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Measurement name cannot be empty")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_measurement_no_fields() {
    let mock_client = MockInfluxDBClient::new();

    let request = CreateMeasurementRequest {
        database: "test_db".to_string(),
        name: "new_measurement".to_string(),
        fields: vec![],
        tags: vec![],
    };

    let result = create_measurement(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("At least one field is required")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_measurement_duplicate_field_names() {
    let mock_client = MockInfluxDBClient::new();

    let request = CreateMeasurementRequest {
        database: "test_db".to_string(),
        name: "new_measurement".to_string(),
        fields: vec![
            Field {
                name: "value".to_string(),
                field_type: FieldType::Float,
            },
            Field {
                name: "value".to_string(),
                field_type: FieldType::Integer,
            },
        ],
        tags: vec![],
    };

    let result = create_measurement(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Duplicate field names are not allowed")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_create_measurement_duplicate_tag_names() {
    let mock_client = MockInfluxDBClient::new();

    let request = CreateMeasurementRequest {
        database: "test_db".to_string(),
        name: "new_measurement".to_string(),
        fields: vec![
            Field {
                name: "value".to_string(),
                field_type: FieldType::Float,
            },
        ],
        tags: vec![
            Tag {
                name: "host".to_string(),
                value: "server1".to_string(),
            },
            Tag {
                name: "host".to_string(),
                value: "server2".to_string(),
            },
        ],
    };

    let result = create_measurement(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Duplicate tag names are not allowed")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_delete_measurement_success() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_delete_measurement()
        .with(eq("test_db"), eq("cpu"))
        .times(1)
        .returning(|_, _| Ok(()));

    let request = DeleteMeasurementRequest {
        database: "test_db".to_string(),
        name: "cpu".to_string(),
        confirm: true,
    };

    let result = delete_measurement(&mock_client, request).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_measurement_without_confirmation() {
    let mock_client = MockInfluxDBClient::new();

    let request = DeleteMeasurementRequest {
        database: "test_db".to_string(),
        name: "cpu".to_string(),
        confirm: false,
    };

    let result = delete_measurement(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::ValidationError(msg) => assert!(msg.contains("Confirmation required for measurement deletion")),
        _ => panic!("Expected ValidationError"),
    }
}

#[tokio::test]
async fn test_delete_measurement_error() {
    let mut mock_client = MockInfluxDBClient::new();
    mock_client
        .expect_delete_measurement()
        .with(eq("test_db"), eq("cpu"))
        .times(1)
        .returning(|_, _| Err(AppError::DatabaseError("Measurement not found".to_string())));

    let request = DeleteMeasurementRequest {
        database: "test_db".to_string(),
        name: "cpu".to_string(),
        confirm: true,
    };

    let result = delete_measurement(&mock_client, request).await;
    
    assert!(result.is_err());
    match result.unwrap_err() {
        AppError::DatabaseError(msg) => assert_eq!(msg, "Measurement not found"),
        _ => panic!("Expected DatabaseError"),
    }
} 