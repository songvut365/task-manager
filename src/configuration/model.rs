use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct AppConfig {
    pub http_server: HttpServer,
    pub database: Database,
    pub redis: Redis,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct HttpServer {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Database {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: String,
    pub database: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Redis {
    pub host: String,
    pub port: String,
    pub database: String,
    pub timeout: u64,
}
