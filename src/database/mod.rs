use sea_orm::DatabaseConnection;
use std::{
    env::{self, VarError},
    error::Error,
    result::Result,
};
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

pub struct DatabaseConfig {
    pub connection_url: String,
}

impl DatabaseConfig {
    pub fn new(url: String) -> Self {
        DatabaseConfig {
            connection_url: url,
        }
    }
}

impl DatabaseConfig {
    pub fn from_env(connection_key: String) -> Result<Self, Box<dyn Error>> {
        let connection_url = match env::var(&connection_key) {
            Ok(v) => v,
            Err(VarError::NotPresent) => {
                return Err("Connection URL not found".into());
            }
            Err(VarError::NotUnicode(v)) => {
                return Err(<Box<dyn Error>>::from(format!(
                    "Connection URL is not a valid string go: {:?}",
                    v
                )));
            }
        };

        Ok(DatabaseConfig { connection_url })
    }
}
