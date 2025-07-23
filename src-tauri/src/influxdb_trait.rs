
use async_trait::async_trait;
use crate::models::{DatabaseInfo, QueryResult};
use crate::error::AppError;

#[async_trait]
pub trait InfluxDB {
    async fn get_databases(&self) -> Result<Vec<String>, AppError>;
    async fn get_database_info(&self, db_name: &str) -> Result<DatabaseInfo, AppError>;
    async fn query(&self, query: &str) -> Result<Vec<QueryResult>, AppError>;
}
