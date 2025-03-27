use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug)]
pub struct SidecarState(u32);

impl SidecarState {
    pub fn default() -> Self {
        Self(0)
    }
    pub fn set(&mut self, pid: u32) {
        self.0 = pid;
    }
    pub fn get(&mut self) -> u32 {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq, EnumString)]
pub enum AccessMode {
    #[serde(rename = "auto")]
    #[strum(serialize = "auto")]
    Auto,
    #[serde(rename = "proxy")]
    #[strum(serialize = "proxy")]
    Proxy,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq, EnumString)]
pub enum BindMode {
    #[serde(rename = "socks")]
    #[strum(serialize = "socks")]
    Socks,
    #[serde(rename = "http")]
    #[strum(serialize = "http")]
    Http,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq, EnumString)]
pub enum ProtocolMode {
    #[serde(rename = "quic")]
    #[strum(serialize = "quic")]
    Quic,
    #[serde(rename = "tcp-udp")]
    #[strum(serialize = "tcp-udp")]
    TcpUdp,
}
