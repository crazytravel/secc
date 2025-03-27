use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AddrInfo {
    pub host: String,
    pub port: u16,
}

impl AddrInfo {
    pub fn new(host: String, port: u16) -> Self {
        Self { host, port }
    }

    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            host: json["host"].as_str().unwrap().to_string(),
            port: json["port"].as_u64().unwrap() as u16,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenConfig {
    pub socks_ip: String,
    pub socks_port: u16,
    pub http_ip: String,
    pub http_port: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListenConfigOption {
    pub socks_config: Option<AddrInfo>,
    pub http_config: Option<AddrInfo>,
}

impl ListenConfigOption {
    pub fn new(socks_config: Option<AddrInfo>, http_config: Option<AddrInfo>) -> Self {
        Self {
            socks_config,
            http_config,
        }
    }
}
