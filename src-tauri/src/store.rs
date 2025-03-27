use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
    path::PathBuf,
};

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::AppHandle;
use tauri_plugin_http::reqwest;
use tauri_plugin_store::{StoreExt, resolve_store_path};

use crate::server::AddrInfo;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub proxy_list: Vec<String>,
    pub direct_list: Vec<String>,
}

pub const CONFIG_PATH: &str = "config.json";
pub const SERVER_ADDR: &str = "server";
pub const SOCKS_ADDR: &str = "socks";
pub const HTTP_ADDR: &str = "http";
pub const ACCESS_MODE: &str = "access_mode";
pub const BIND_MODE: &str = "bind_mode";
pub const PROTOCOL_MODE: &str = "protocol_mode";

pub const PROXY_RULES_PATH: &str = "proxy_list.txt";
pub const DIRECT_RULES_PATH: &str = "direct_list.txt";
pub const PROXY_RULES_URL: &str =
    "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/proxy-list.txt"; // https://raw.githubusercontent.com/Loyalsoldier/v2ray-rules-dat/release/proxy-list.txt
pub const CERT_PATH: &str = "cert.pem";
pub const CERT_KEY_PATH: &str = "cert.key.pem";

pub async fn init_all(app: &AppHandle) -> Result<(), Error> {
    let path = resolve_store_path(app, "")?;
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all(&path)?;
    }
    init_proxy_rules(app).await?;
    init_direct_rules(app)?;
    init_cert(app)?;
    Ok(())
}

pub async fn init_proxy_rules(app: &AppHandle) -> Result<(), Error> {
    let path = resolve_store_path(app, PROXY_RULES_PATH)?;
    if !std::path::Path::new(&path).exists() {
        let response = reqwest::get(PROXY_RULES_URL).await?;
        let byte_data = response.bytes().await?;
        println!("proxy_list_path: {:?}", path);
        let mut file = File::create(path)?;
        file.write_all(&byte_data)?;
        file.flush()?;
    }
    Ok(())
}

pub fn init_direct_rules(app: &AppHandle) -> Result<(), Error> {
    let path = resolve_store_path(app, DIRECT_RULES_PATH)?;
    if !std::path::Path::new(&path).exists() {
        let mut file = File::create(path)?;
        file.write_all(b"localhost\n")?;
        file.flush()?;
    }
    Ok(())
}

pub fn init_cert(app: &AppHandle) -> Result<(), Error> {
    let cert_path = resolve_store_path(app, CERT_PATH)?;
    let cert_key_path = resolve_store_path(app, CERT_KEY_PATH)?;
    if !std::path::Path::new(&cert_path).exists() {
        File::create(cert_path)?;
    }
    if !std::path::Path::new(&cert_key_path).exists() {
        File::create(cert_key_path)?;
    }
    Ok(())
}

pub fn get_path(app: &AppHandle, sub_path: &str) -> Result<PathBuf, Error> {
    let path = resolve_store_path(app, sub_path)?;
    Ok(path)
}

pub fn set_address(app: &AppHandle, config_key: &str, config_value: AddrInfo) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(config_key, json!(config_value));
    Ok(())
}

pub fn get_address(app: &AppHandle, config_key: &str) -> Result<Option<AddrInfo>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(config_key);
    if let Some(data) = data {
        let config = AddrInfo::from_json(&data);
        return Ok(Some(config));
    }
    Ok(None)
}

pub fn set_mode(app: &AppHandle, mode: &str, mode_value: &str) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(mode, mode_value);
    Ok(())
}

pub fn get_mode(app: &AppHandle, mode: &str) -> Result<Option<String>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(mode);
    if let Some(data) = data {
        let mode_value = data.as_str().unwrap().to_string();
        return Ok(Some(mode_value));
    }
    Ok(None)
}

pub fn set_rules(app: &AppHandle, rule_path: &str, rules: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, rule_path)?;
    let mut file = File::create(path)?;
    file.write_all(rules.as_bytes())?;
    file.flush()?;
    Ok(())
}

pub fn get_rules(app: &AppHandle, rule_path: &str) -> Result<String, Error> {
    let path = resolve_store_path(app, rule_path)?;
    let file = File::open(path)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}

// append domain list to the direct list with a newline
pub fn _append_direct_rules(app: &AppHandle, domain: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, DIRECT_RULES_PATH)?;
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(domain.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

// append a domain to the proxy list with a newline
pub fn _append_proxy_rules(app: &AppHandle, domain: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, PROXY_RULES_PATH)?;
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(domain.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

pub fn set_cert(app: &AppHandle, cert_sub_path: &str, cert: &str) -> Result<(), Error> {
    let cert_path = resolve_store_path(app, cert_sub_path)?;
    let mut cert_file = File::create(cert_path)?;
    cert_file.write_all(cert.as_bytes())?;
    Ok(())
}

pub fn get_cert(app: &AppHandle, cert_sub_path: &str) -> Result<String, Error> {
    let cert_path = resolve_store_path(app, cert_sub_path)?;
    let mut cert_file = File::open(cert_path)?;
    let mut cert = String::new();
    cert_file.read_to_string(&mut cert)?;
    Ok(cert)
}
