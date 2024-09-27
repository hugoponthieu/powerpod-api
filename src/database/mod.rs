use sea_orm::DatabaseConnection;
use std::{error::Error, result::Result};
pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new(connectionString: &str) -> Result<Self, Box<dyn Error + 'static>> {
        let connection = sea_orm::Database::connect(connectionString).await?;
        Ok(Database {
            connection: connection,
        })
    }
}
