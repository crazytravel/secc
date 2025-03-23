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

use crate::server::ServerConfig;

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub proxy_list: Vec<String>,
    pub direct_list: Vec<String>,
}

pub const CONFIG_PATH: &str = "config.json";
pub const SERVER_KEY: &str = "server";
pub const SOCKS_KEY: &str = "socks";
pub const HTTP_KEY: &str = "http";
pub const ACCESS_MODE: &str = "access_mode";
pub const BIND_MODE: &str = "bind_mode";

pub const PROXY_LIST_PATH: &str = "proxy_list.txt";
pub const DIRECT_LIST_PATH: &str = "direct_list.txt";
pub const PROXY_LIST_URL: &str =
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

pub fn get_proxy_list_path(app: &AppHandle) -> Result<PathBuf, Error> {
    let path = resolve_store_path(app, PROXY_LIST_PATH)?;
    Ok(path)
}

pub fn get_direct_list_path(app: &AppHandle) -> Result<PathBuf, Error> {
    let path = resolve_store_path(app, DIRECT_LIST_PATH)?;
    Ok(path)
}

pub fn get_cert_path(app: &AppHandle) -> Result<PathBuf, Error> {
    let path = resolve_store_path(app, CERT_PATH)?;
    Ok(path)
}

pub fn set_server_config(app: &AppHandle, server_config: ServerConfig) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(SERVER_KEY, json!(server_config));
    Ok(())
}

pub fn get_server_config(app: &AppHandle) -> Result<Option<ServerConfig>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SERVER_KEY);
    if let Some(data) = data {
        let server_config = ServerConfig::from_json(&data);
        return Ok(Some(server_config));
    }
    Ok(None)
}

pub fn set_socks_config(app: &AppHandle, socks_config: ServerConfig) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(SOCKS_KEY, json!(socks_config));
    Ok(())
}

pub fn get_socks_config(app: &AppHandle) -> Result<Option<ServerConfig>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SOCKS_KEY);
    if let Some(data) = data {
        let socks_config = ServerConfig::from_json(&data);
        return Ok(Some(socks_config));
    }
    Ok(None)
}

pub fn set_http_config(app: &AppHandle, http_config: ServerConfig) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(HTTP_KEY, json!(http_config));
    Ok(())
}

pub fn get_http_config(app: &AppHandle) -> Result<Option<ServerConfig>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(HTTP_KEY);
    if let Some(data) = data {
        let http_config = ServerConfig::from_json(&data);
        return Ok(Some(http_config));
    }
    Ok(None)
}

pub fn set_access_mode(app: &AppHandle, access_mode: &str) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(ACCESS_MODE, access_mode);
    Ok(())
}

pub fn get_access_mode(app: &AppHandle) -> Result<Option<String>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(ACCESS_MODE);
    if let Some(data) = data {
        let access_mode = data.as_str().unwrap().to_string();
        return Ok(Some(access_mode));
    }
    Ok(None)
}

pub fn set_bind_mode(app: &AppHandle, bind_mode: &str) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(BIND_MODE, bind_mode);
    Ok(())
}

pub fn get_bind_mode(app: &AppHandle) -> Result<Option<String>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(BIND_MODE);
    if let Some(data) = data {
        let bind_mode = data.as_str().unwrap().to_string();
        return Ok(Some(bind_mode));
    }
    Ok(None)
}

pub async fn init_proxy_rules(app: &AppHandle) -> Result<(), Error> {
    let path = resolve_store_path(app, PROXY_LIST_PATH)?;
    if !std::path::Path::new(&path).exists() {
        let response = reqwest::get(PROXY_LIST_URL).await?;
        let byte_data = response.bytes().await?;
        println!("proxy_list_path: {:?}", path);
        let mut file = File::create(path)?;
        file.write_all(&byte_data)?;
    }
    Ok(())
}

pub fn set_proxy_rules(app: &AppHandle, proxy_rules: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, PROXY_LIST_PATH)?;
    let mut file = File::create(path)?;
    file.write_all(proxy_rules.as_bytes())?;
    Ok(())
}

pub fn get_proxy_rules(app: &AppHandle) -> Result<String, Error> {
    let path = resolve_store_path(app, PROXY_LIST_PATH)?;
    let file = File::open(path)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}

pub fn init_direct_rules(app: &AppHandle) -> Result<(), Error> {
    let path = resolve_store_path(app, DIRECT_LIST_PATH)?;
    if !std::path::Path::new(&path).exists() {
        let mut file = File::create(path)?;
        file.write_all(b"localhost\n")?;
    }
    Ok(())
}

pub fn get_direct_rules(app: &AppHandle) -> Result<String, Error> {
    let path = resolve_store_path(app, DIRECT_LIST_PATH)?;
    let file = File::open(path)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}

pub fn set_direct_rules(app: &AppHandle, direct_rules: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, DIRECT_LIST_PATH)?;
    println!("direct_rules_path: {:?}", path);
    let mut file = File::create(path)?;
    file.write_all(direct_rules.as_bytes())?;
    Ok(())
}

// append domain list to the direct list with a newline
pub fn _append_direct_rules(app: &AppHandle, domain: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, DIRECT_LIST_PATH)?;
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(domain.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

// append a domain to the proxy list with a newline
pub fn _append_proxy_rules(app: &AppHandle, domain: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, PROXY_LIST_PATH)?;
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(domain.as_bytes())?;
    file.write_all(b"\n")?;
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

pub fn set_cert(app: &AppHandle, cert: &str) -> Result<(), Error> {
    let cert_path = resolve_store_path(app, CERT_PATH)?;
    let mut cert_file = File::create(cert_path)?;
    cert_file.write_all(cert.as_bytes())?;
    Ok(())
}

pub fn set_cert_key(app: &AppHandle, cert_key: &str) -> Result<(), Error> {
    let cert_key_path = resolve_store_path(app, CERT_KEY_PATH)?;
    let mut cert_key_file = File::create(cert_key_path)?;
    cert_key_file.write_all(cert_key.as_bytes())?;
    Ok(())
}

pub fn get_cert(app: &AppHandle) -> Result<String, Error> {
    let cert_path = resolve_store_path(app, CERT_PATH)?;
    let mut cert_file = File::open(cert_path)?;
    let mut cert = String::new();
    cert_file.read_to_string(&mut cert)?;
    Ok(cert)
}

pub fn get_cert_key(app: &AppHandle) -> Result<String, Error> {
    let cert_key_path = resolve_store_path(app, CERT_KEY_PATH)?;
    let mut cert_key_file = File::open(cert_key_path)?;
    let mut cert_key = String::new();
    cert_key_file.read_to_string(&mut cert_key)?;
    Ok(cert_key)
}
