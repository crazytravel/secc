use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServerInfo {
    pub host: String,
    pub quic_port: u16,
    pub tcp_port: Option<u16>,
    pub cert: String,
    pub cert_key: String,
    pub alias: Option<String>,
}

impl ServerInfo {
    pub fn from_json(json: &serde_json::Value) -> Self {
        Self {
            host: json["host"].as_str().unwrap().to_string(),
            quic_port: json["quic_port"].as_u64().unwrap() as u16,
            tcp_port: json["tcp_port"].as_u64().map(|port| port as u16),
            cert: json["cert"].as_str().unwrap().to_string(),
            cert_key: json["cert_key"].as_str().unwrap().to_string(),
            alias: json["alias"].as_str().map(|alias| alias.to_string()),
        }
    }

    pub fn from_json_array(json: &serde_json::Value) -> Vec<Self> {
        let mut result = Vec::new();
        for item in json.as_array().unwrap() {
            result.push(Self::from_json(item));
        }
        result
    }
}

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
