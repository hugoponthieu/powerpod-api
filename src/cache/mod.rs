use std::error::Error;

use redis::{Commands, Connection};

pub struct Cache {
    pub connection: Connection,
}

impl Cache {
    pub fn new(connection_url: &str) -> Result<Self, Box<dyn Error>> {
        let client = redis::Client::open(connection_url)?;
        let connection = client.get_connection()?;
        Ok(Cache { connection })
    }
    pub fn save(
        &mut self,
        key: &str,
        value: &str,
        seconds: Option<u64>,
    ) -> Result<(), Box<dyn Error>> {
        let ttl = seconds.unwrap_or(600);
        let _ = self
            .connection
            .set_ex::<&str, &str, usize>(key, value, ttl)?;
        Ok(())
    }

    pub fn get(&mut self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let value: Option<String> = self.connection.get(key)?;
        Ok(value)
    }

    pub fn invalidate(&mut self, key: &str) -> Result<i32, Box<dyn Error>> {
        let res = self.connection.del(key)?;
        match res {
            0 => return Err("Key not found".into()),
            _ => {}
        }
        Ok(res)
    }
}

pub struct CacheConfig {
    pub connection_url: String,
    pub ttl: u64,
}

impl CacheConfig {
    pub fn new(url: String, ttl: u64) -> Self {
        CacheConfig {
            connection_url: url,
            ttl: ttl,
        }
    }
}

impl CacheConfig {
    pub fn from_env(connection_key: String, ttl_key: String) -> Result<Self, Box<dyn Error>> {
        let connection_url = match std::env::var(&connection_key) {
            Ok(v) => v,
            Err(std::env::VarError::NotPresent) => {
                return Err("Connection URL not found".into());
            }
            Err(std::env::VarError::NotUnicode(v)) => {
                return Err(Box::from(format!(
                    "Connection URL is not a valid string go: {:?}",
                    v
                )));
            }
        };
        let ttl = match std::env::var(&ttl_key) {
            Ok(v) => v.parse::<u64>()?,
            Err(std::env::VarError::NotPresent) => {
                return Err("TTL not found".into());
            }
            Err(std::env::VarError::NotUnicode(v)) => {
                return Err(Box::from(format!("TTL is not a valid string go: {:?}", v)));
            }
        };
        Ok(CacheConfig {
            connection_url,
            ttl,
        })
    }
}
