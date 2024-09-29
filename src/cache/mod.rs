mod items;

use std::error::Error;

use items::Items;
use redis::{Commands, Connection};
use serde_json::Value;

pub struct Cache {
    pub connection: Connection,
    pub ttl: u64,
}

impl Cache {
    pub fn new(cache_config: CacheConfig) -> Result<Self, Box<dyn Error>> {
        let client = redis::Client::open(cache_config.connection_url)?;
        let connection = client.get_connection()?;
        Ok(Cache {
            connection,
            ttl: cache_config.ttl,
        })
    }
    pub fn save(&mut self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
        let saved_value = serde_json::to_string(&value)?;
        let _ = self
            .connection
            .set_ex::<&str, &str, usize>(key, saved_value.as_str(), self.ttl)?;
        Ok(())
    }

    pub fn m_save(&mut self, items: &Items) -> Result<(), Box<dyn Error>> {
        for item in items.iter() {
            let key = item.0.as_str();
            let value = item.1.as_str();
            self.save(key, value.to_owned())?;
        }
        Ok(())
    }
    pub fn get(&mut self, key: &str) -> Result<Option<Value>, Box<dyn Error>> {
        let value: Option<String> = self.connection.get(key)?;
        match value {
            None => Ok(None),
            Some(v) => {
                let parsed_value: Value = serde_json::from_str(v.as_str())?;
                return Ok(Some(parsed_value));
            }
        }
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
