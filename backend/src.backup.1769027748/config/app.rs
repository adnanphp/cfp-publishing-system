// backend/src/config/app.rs
use serde::Deserialize;
use std::net::SocketAddr;

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationSettings {
    pub host: String,
    pub port: u16,
    pub base_url: String,
    pub frontend_url: String,
    pub hmac_secret: String,
    pub session_cookie_name: String,
    pub session_duration_hours: u64,
    pub enable_swagger: bool,
    pub log_level: String,
    pub workers: usize,
}

impl ApplicationSettings {
    pub fn socket_address(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port)
            .parse()
            .expect("Invalid socket address")
    }
    
    pub fn session_duration_seconds(&self) -> u64 {
        self.session_duration_hours * 3600
    }
}
