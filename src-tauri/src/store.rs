use std::{
    fs::{File, OpenOptions},
    io::{BufReader, Read, Write},
};

use anyhow::Error;
use tauri::AppHandle;

pub const PROXY_LIST_PATH: &str = "proxy_list.txt";
pub const DIRECT_LIST_PATH: &str = "direct_list.txt";
pub const PROXY_LIST_URL: &str =
    "https://cdn.jsdelivr.net/gh/Loyalsoldier/v2ray-rules-dat@release/proxy-list.txt"; // https://raw.githubusercontent.com/Loyalsoldier/v2ray-rules-dat/release/proxy-list.txt
pub const cert_path: &str = "cert.pem";
pub const cert_key_path: &str = "cert.key.pem";

pub fn init_proxy_list(app: &AppHandle) -> Result<(), Error> {
    let proxy_list = reqwest::blocking::get(PROXY_LIST_URL)?.text()?;
    let mut file = File::create(PROXY_LIST_PATH)?;
    file.write_all(proxy_list.as_bytes())?;
    Ok(())
}

pub fn init_direct_list() -> Result<(), Error> {
    File::create(DIRECT_LIST_PATH)?;
    Ok(())
}

pub fn read_proxy_list() -> Result<String, Error> {
    let file = File::open(PROXY_LIST_PATH)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}

pub fn read_direct_list() -> Result<String, Error> {
    let file = File::open(DIRECT_LIST_PATH)?;
    let mut content = String::new();
    BufReader::new(file).read_to_string(&mut content)?;
    Ok(content)
}

// append domain list to the direct list with a newline
pub fn append_direct_list(domain: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().append(true).open(DIRECT_LIST_PATH)?;
    file.write_all(domain.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

// append a domain to the proxy list with a newline
pub fn append_proxy_list(domain: &str) -> Result<(), Error> {
    let mut file = OpenOptions::new().append(true).open(PROXY_LIST_PATH)?;
    file.write_all(domain.as_bytes())?;
    file.write_all(b"\n")?;
    Ok(())
}

pub fn init_cert() -> Result<(), Error> {
    File::create(cert_path)?;
    File::create(cert_key_path)?;
    Ok(())
}
