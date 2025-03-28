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

use crate::{
    server::{AddrInfo, ServerInfo},
    state::{AccessMode, BindMode, ProtocolMode},
};

#[derive(Serialize, Deserialize)]
pub struct Domain {
    pub proxy_list: Vec<String>,
    pub direct_list: Vec<String>,
}

pub const CONFIG_PATH: &str = "config.json";
pub const ACTIVE_SERVER: &str = "active_server";
pub const SERVERS: &str = "servers";
pub const SOCKS_ADDR: &str = "socks";
pub const HTTP_ADDR: &str = "http";
pub const ACCESS_MODE: &str = "access_mode";
pub const BIND_MODE: &str = "bind_mode";
pub const PROTOCOL_MODE: &str = "protocol_mode";
pub const COMMUNITY_RULES: &str = "rules_url";

pub const PROXY_RULES_PATH: &str = "proxy_list.txt";
pub const CUSTOM_PROXY_RULES_PATH: &str = "custom_proxy_list.txt";
pub const COMMUNITY_PROXY_RULES_PATH: &str = "community_proxy_list.txt";
pub const DIRECT_RULES_PATH: &str = "direct_list.txt";
pub const COMMUNITY_RULES_URL: &str =
    "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/proxy-list.txt"; // https://raw.githubusercontent.com/Loyalsoldier/v2ray-rules-dat/release/proxy-list.txt
pub const CERT_PATH: &str = "cert.pem";
pub const CERT_KEY_PATH: &str = "cert.key.pem";

pub fn init_all(app: &AppHandle) -> Result<(), Error> {
    let path = resolve_store_path(app, "")?;
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all(&path)?;
    }
    let bind_socks_addr = get_address(app, SOCKS_ADDR)?;
    if bind_socks_addr.is_none() {
        set_address(
            app,
            SOCKS_ADDR,
            AddrInfo {
                host: "127.0.0.1".to_string(),
                port: 1080,
            },
        )?;
    }
    let bind_http_addr = get_address(app, HTTP_ADDR)?;
    if bind_http_addr.is_none() {
        set_address(
            app,
            HTTP_ADDR,
            AddrInfo {
                host: "127.0.0.1".to_string(),
                port: 1081,
            },
        )?;
    }
    let access_mode = get_str_config(app, ACCESS_MODE)?;
    if access_mode.is_none() {
        set_str_config(app, ACCESS_MODE, AccessMode::Auto.to_string().as_str())?;
    }
    let bind_mode = get_str_config(app, BIND_MODE)?;
    if bind_mode.is_none() {
        set_str_config(app, BIND_MODE, BindMode::Socks.to_string().as_str())?;
    }
    let protocol_mode = get_str_config(app, PROTOCOL_MODE)?;
    if protocol_mode.is_none() {
        set_str_config(app, PROTOCOL_MODE, ProtocolMode::Quic.to_string().as_str())?;
    }
    let rules_url = get_str_config(app, COMMUNITY_RULES)?;
    if rules_url.is_none() {
        set_str_config(app, COMMUNITY_RULES, COMMUNITY_RULES_URL)?;
    }

    init_rules_files(app)?;
    init_cert_files(app)?;

    Ok(())
}

pub async fn load_community_proxy_list(app: &AppHandle) -> Result<(), Error> {
    let url = get_str_config(app, COMMUNITY_RULES)?;
    let req_url = match url {
        Some(url) => url,
        None => COMMUNITY_RULES_URL.to_string(),
    };
    let response = reqwest::get(req_url).await?;
    let byte_data = response.bytes().await?;
    let path = resolve_store_path(app, COMMUNITY_PROXY_RULES_PATH)?;
    let mut file = File::create(&path)?;
    file.write_all(&byte_data)?;
    file.flush()?;

    combine_proxy_rules(app)?;
    Ok(())
}

pub fn init_rules_files(app: &AppHandle) -> Result<(), Error> {
    let proxy_path = resolve_store_path(app, PROXY_RULES_PATH)?;
    if !std::path::Path::new(&proxy_path).exists() {
        println!("proxy_list_path: {:?}", proxy_path);
        File::create(proxy_path)?;
    }
    let direct_path = resolve_store_path(app, DIRECT_RULES_PATH)?;
    if !std::path::Path::new(&direct_path).exists() {
        let mut file = File::create(direct_path)?;
        file.write_all(b"localhost\n")?;
        file.flush()?;
    }
    let custom_proxy_path = resolve_store_path(app, CUSTOM_PROXY_RULES_PATH)?;
    if !std::path::Path::new(&custom_proxy_path).exists() {
        File::create(custom_proxy_path)?;
    }
    let community_proxy_path = resolve_store_path(app, COMMUNITY_PROXY_RULES_PATH)?;
    if !std::path::Path::new(&community_proxy_path).exists() {
        File::create(community_proxy_path)?;
    }

    Ok(())
}

pub fn init_cert_files(app: &AppHandle) -> Result<(), Error> {
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

pub fn get_servers(app: &AppHandle) -> Result<Option<Vec<ServerInfo>>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SERVERS);
    if let Some(data) = data {
        let config = ServerInfo::from_json_array(&data);
        return Ok(Some(config));
    }
    Ok(None)
}

pub fn get_server(app: &AppHandle, host: &str) -> Result<Option<ServerInfo>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SERVERS);
    if let Some(data) = data {
        let config = ServerInfo::from_json_array(&data);
        let server = config.iter().find(|x| x.host == host);
        if let Some(server) = server {
            return Ok(Some(server.clone()));
        }
    }
    Ok(None)
}

pub fn add_server(app: &AppHandle, server: ServerInfo) -> Result<Option<()>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SERVERS);
    if let Some(data) = data {
        let mut config = ServerInfo::from_json_array(&data);
        config.push(server);
        store.set(SERVERS, json!(config));
        return Ok(Some(()));
    }
    let servers = vec![server];
    store.set(SERVERS, json!(servers));
    Ok(None)
}

pub fn update_server(app: &AppHandle, server: ServerInfo) -> Result<Option<()>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SERVERS);
    if let Some(data) = data {
        let mut config = ServerInfo::from_json_array(&data);
        let index = config.iter().position(|x| x.host == server.host);
        if let Some(index) = index {
            config[index] = server;
            store.set(SERVERS, json!(config));
            return Ok(Some(()));
        }
    }
    Ok(None)
}

pub fn delete_server(app: &AppHandle, host: &str) -> Result<Option<()>, Error> {
    let store = app.store(CONFIG_PATH)?;
    let data = store.get(SERVERS);
    if let Some(data) = data {
        let mut config = ServerInfo::from_json_array(&data);
        let index = config.iter().position(|x| x.host == host);
        if let Some(index) = index {
            config.remove(index);
            store.set(SERVERS, json!(config));
            return Ok(Some(()));
        }
    }
    Ok(None)
}

pub fn set_str_config(app: &AppHandle, mode: &str, mode_value: &str) -> Result<(), Error> {
    let store = app.store(CONFIG_PATH)?;
    store.set(mode, mode_value);
    Ok(())
}

pub fn get_str_config(app: &AppHandle, mode: &str) -> Result<Option<String>, Error> {
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
    if rule_path == CUSTOM_PROXY_RULES_PATH {
        combine_proxy_rules(app)?;
    }

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
pub fn _append_direct_rules(app: &AppHandle, rules: &str) -> Result<(), Error> {
    let path = resolve_store_path(app, DIRECT_RULES_PATH)?;
    let mut file = OpenOptions::new().append(true).open(path)?;
    file.write_all(rules.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

pub fn combine_proxy_rules(app: &AppHandle) -> Result<(), Error> {
    let custom_proxy_path = resolve_store_path(app, CUSTOM_PROXY_RULES_PATH)?;
    let community_proxy_path = resolve_store_path(app, COMMUNITY_PROXY_RULES_PATH)?;
    let combine_proxy_path = resolve_store_path(app, PROXY_RULES_PATH)?;

    // Read contents of the first file
    let mut custom_file = File::open(custom_proxy_path)?;
    let mut custom_content = String::new();
    custom_file.read_to_string(&mut custom_content)?;

    // Read contents of the second file
    let mut community_file = File::open(community_proxy_path)?;
    let mut community_content = String::new();
    community_file.read_to_string(&mut community_content)?;

    // Combine contents with a newline separator
    let combined_content = format!(
        "{}\n{}",
        custom_content.trim_end(),
        community_content.trim_end()
    );

    // Write the combined content to the output file
    let mut output_file = File::create(combine_proxy_path)?;
    output_file.write_all(combined_content.as_bytes())?;
    output_file.flush()?;

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
