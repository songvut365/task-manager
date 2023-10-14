use crate::configuration::model::Redis;
use redis::{Commands, Connection, RedisError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::{error::Error, time::Duration};

pub struct Client {
    config: Redis,
    connection: Option<Connection>,
}

impl Client {
    pub fn new(config: Redis) -> Self {
        Client {
            config,
            connection: None,
        }
    }

    pub fn connect_redis(&mut self) -> Result<(), RedisError> {
        let redis_url = format!(
            "redis://{}:{}/{}",
            self.config.host, self.config.port, self.config.database
        );

        let redis_client = match redis::Client::open(redis_url) {
            Ok(client) => client,
            Err(err) => return Err(err),
        };

        let timeout = Duration::new(self.config.timeout, 0);
        let connection = match redis_client.get_connection_with_timeout(timeout) {
            Ok(connection) => connection,
            Err(err) => return Err(err),
        };

        self.connection = Some(connection);
        Ok(())
    }

    pub fn get<T>(&mut self, key: String) -> Result<T, Box<dyn Error>>
    where
        T: DeserializeOwned,
    {
        let connection = self
            .connection
            .as_mut()
            .expect("redis connection has not been initiated");

        let data: String = match connection.get(key) {
            Ok(result) => result,
            Err(err) => return Err(Box::new(err)),
        };

        let result: T = serde_json::from_str(&data)?;
        Ok(result)
    }

    pub fn set<T>(&mut self, key: String, value: &T) -> Result<(), Box<dyn Error>>
    where
        T: Serialize,
    {
        let connection = self
            .connection
            .as_mut()
            .expect("redis connection has not been initiated");

        let value_json = match serde_json::to_string(value) {
            Ok(result) => result,
            Err(err) => return Err(Box::new(err)),
        };

        let ttl = std::time::Duration::from_secs(5 * 60) // 5 minutes
            .as_secs()
            .try_into()
            .unwrap();
        connection.set_ex(key, value_json, ttl)?;
        Ok(())
    }

    pub fn delete(&mut self, key: String) -> Result<(), Box<dyn Error>> {
        let connection = self
            .connection
            .as_mut()
            .expect("redis connection has not been initiated");

        connection.del(key)?;
        Ok(())
    }
}
