use crate::models::{ApiResponse, Measurement, FieldKey};
use crate::error::AppError;
use crate::influxdb::InfluxDBService;
use serde_json::Value;

/// List all measurements from a specific database
pub async fn list_measurements(
    service: &InfluxDBService,
    database: &str
) -> Result<ApiResponse<Vec<String>>, String> {
    tracing::info!("[BE] measurement::list_measurements called for database: {}", database);
    
    match service.get_measurements(database).await {
        Ok(measurements) => {
            tracing::info!("[BE] measurement::list_measurements succeeded, found {} measurements", measurements.len());
            Ok(ApiResponse {
                success: true,
                data: Some(measurements),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] measurement::list_measurements failed with error: {}", e);
            Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            })
        }
    }
}

/// Get detailed information about a specific measurement
pub async fn get_measurement_info(
    service: &InfluxDBService,
    database: &str,
    measurement: &str
) -> Result<ApiResponse<Measurement>, String> {
    tracing::info!("[BE] measurement::get_measurement_info called for database: {}, measurement: {}", 
                  database, measurement);
    
    // First, check if the measurement exists
    let measurements = match service.get_measurements(database).await {
        Ok(measurements) => measurements,
        Err(e) => {
            tracing::error!("[BE] Failed to get measurements: {}", e);
            return Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            });
        }
    };
    
    if !measurements.contains(&measurement.to_string()) {
        tracing::error!("[BE] Measurement '{}' not found in database '{}'", measurement, database);
        return Ok(ApiResponse {
            success: false,
            data: None,
            error: Some(format!("Measurement '{}' not found in database '{}'", measurement, database)),
        });
    }
    
    // Get field keys
    let field_keys = match get_field_keys(service, database, measurement).await {
        Ok(keys) => keys,
        Err(e) => {
            tracing::error!("[BE] Failed to get field keys: {}", e);
            return Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            });
        }
    };
    
    // Get tag keys
    let tag_keys = match get_tag_keys(service, database, measurement).await {
        Ok(keys) => keys,
        Err(e) => {
            tracing::error!("[BE] Failed to get tag keys: {}", e);
            return Ok(ApiResponse {
                success: false,
                data: None,
                error: Some(e.to_string()),
            });
        }
    };
    
    // Create measurement info
    let measurement_info = Measurement {
        name: measurement.to_string(),
        tag_keys,
        field_keys,
        series_count: 0, // This would require additional query to get accurate count
    };
    
    Ok(ApiResponse {
        success: true,
        data: Some(measurement_info),
        error: None,
    })
}

/// Create a new measurement with specified fields and tags
pub async fn create_measurement(
    service: &InfluxDBService,
    database: &str,
    measurement: &str,
    fields: Vec<(String, String, Value)>, // (name, type, value)
    tags: Option<Vec<(String, String)>>, // (name, value)
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] measurement::create_measurement called for database: {}, measurement: {}", 
                  database, measurement);
    
    // Validate measurement name
    if measurement.is_empty() {
        return Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some("Measurement name cannot be empty".to_string()),
        });
    }
    
    // Validate fields
    if fields.is_empty() {
        return Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some("At least one field must be specified".to_string()),
        });
    }
    
    // Build the line protocol data
    let mut line_protocol = format!("{}", measurement);
    
    // Add tags if provided
    if let Some(tags) = &tags {
        for (name, value) in tags {
            line_protocol.push_str(&format!(",{}={}", name, value));
        }
    }
    
    // Add fields
    line_protocol.push_str(" ");
    let mut first = true;
    for (name, field_type, value) in &fields {
        if !first {
            line_protocol.push_str(",");
        }
        
        // Format the value based on its type
        match field_type.as_str() {
            "string" => {
                let string_value = value.as_str().unwrap_or("").replace("\"", "\\\"");
                line_protocol.push_str(&format!("{}=\"{}\"", name, string_value));
            },
            "float" => {
                let float_value = value.as_f64().unwrap_or(0.0);
                line_protocol.push_str(&format!("{}={}", name, float_value));
            },
            "integer" => {
                let int_value = value.as_i64().unwrap_or(0);
                line_protocol.push_str(&format!("{}={}i", name, int_value));
            },
            "boolean" => {
                let bool_value = value.as_bool().unwrap_or(false);
                line_protocol.push_str(&format!("{}={}", name, bool_value));
            },
            _ => {
                // Default to string for unknown types
                let string_value = value.as_str().unwrap_or("").replace("\"", "\\\"");
                line_protocol.push_str(&format!("{}=\"{}\"", name, string_value));
            }
        }
        
        first = false;
    }
    
    // Execute the write query
    let query = format!("INSERT INTO {} {}", database, line_protocol);
    match service.query(&query).await {
        Ok(_) => {
            tracing::info!("[BE] measurement::create_measurement succeeded for database: {}, measurement: {}", 
                          database, measurement);
            Ok(ApiResponse {
                success: true,
                data: Some(true),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] measurement::create_measurement failed for database: {}, measurement: {}: {}", 
                           database, measurement, e);
            Ok(ApiResponse {
                success: false,
                data: Some(false),
                error: Some(e.to_string()),
            })
        }
    }
}

/// Delete an existing measurement
pub async fn delete_measurement(
    service: &InfluxDBService,
    database: &str,
    measurement: &str,
) -> Result<ApiResponse<bool>, String> {
    tracing::info!("[BE] measurement::delete_measurement called for database: {}, measurement: {}", 
                  database, measurement);
    
    // Validate measurement name
    if measurement.is_empty() {
        return Ok(ApiResponse {
            success: false,
            data: Some(false),
            error: Some("Measurement name cannot be empty".to_string()),
        });
    }
    
    // Build the query
    let query = format!("DROP MEASUREMENT \"{}\"", measurement);
    
    // Execute the query with the specified database
    match service.query_with_database(&query, database).await {
        Ok(_) => {
            tracing::info!("[BE] measurement::delete_measurement succeeded for database: {}, measurement: {}", 
                          database, measurement);
            Ok(ApiResponse {
                success: true,
                data: Some(true),
                error: None,
            })
        },
        Err(e) => {
            tracing::error!("[BE] measurement::delete_measurement failed for database: {}, measurement: {}: {}", 
                           database, measurement, e);
            Ok(ApiResponse {
                success: false,
                data: Some(false),
                error: Some(e.to_string()),
            })
        }
    }
}

// Helper function to get field keys for a measurement
async fn get_field_keys(
    service: &InfluxDBService,
    database: &str,
    measurement: &str,
) -> Result<Vec<FieldKey>, AppError> {
    let query = format!("SHOW FIELD KEYS FROM \"{}\"", measurement);
    let result = service.query_with_database(&query, database).await?;
    
    let mut field_keys = Vec::new();
    for series in result.series {
        for row in series.values {
            if row.len() >= 2 {
                if let (Some(name), Some(field_type)) = (row[0].as_str(), row[1].as_str()) {
                    field_keys.push(FieldKey {
                        name: name.to_string(),
                        field_type: field_type.to_string(),
                    });
                }
            }
        }
    }
    
    Ok(field_keys)
}

// Helper function to get tag keys for a measurement
async fn get_tag_keys(
    service: &InfluxDBService,
    database: &str,
    measurement: &str,
) -> Result<Vec<String>, AppError> {
    let query = format!("SHOW TAG KEYS FROM \"{}\"", measurement);
    let result = service.query_with_database(&query, database).await?;
    
    let mut tag_keys = Vec::new();
    for series in result.series {
        for row in series.values {
            if let Some(tag_key) = row.get(0).and_then(|v| v.as_str()) {
                tag_keys.push(tag_key.to_string());
            }
        }
    }
    
    Ok(tag_keys)
}