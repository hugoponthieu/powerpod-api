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
