use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager};

use crate::{
    server::{AddrInfo, ListenConfig, ListenConfigOption, ServerInfo},
    shell,
    state::{AccessMode, AgentState, BindMode, ProtocolMode},
    store::{self, HTTP_ADDR, SOCKS_ADDR},
    tray::{self},
};

#[tauri::command]
pub fn open_secc(app: AppHandle) {
    if let Err(err) = shell::kill_sidecar(&app) {
        eprintln!("close secc error: {:?}", err);
    }
    let app_handle = app.clone();
    let access_mode = get_access_mode(app_handle.clone());
    let bind_mode = get_bind_mode(app_handle.clone());
    match bind_mode {
        BindMode::Socks => {
            shell::switch_to_socks(app_handle.clone());
        }
        BindMode::Http => {
            shell::switch_to_http(app_handle.clone());
        }
    }
    let protocol_mode = get_protocol_mode(app_handle.clone());

    tauri::async_runtime::spawn(async move {
        shell::call_sidecar(&app_handle, access_mode, protocol_mode);
    });
    {
        let agent_state = app.state::<Mutex<AgentState>>();
        let mut agent_state = agent_state.lock().unwrap();
        agent_state.set(true);
    }
}

#[tauri::command]
pub fn close_secc(app: AppHandle) {
    if let Err(err) = shell::kill_sidecar(&app) {
        eprintln!("close secc error: {:?}", err);
    }
    shell::switch_to_direct(app.clone());
    tray::change_tray_icon(&app, false).unwrap();
    {
        let agent_state = app.state::<Mutex<AgentState>>();
        let mut agent_state = agent_state.lock().unwrap();
        agent_state.set(false);
    }
}

#[tauri::command]
pub fn switch_access_mode(app: AppHandle, access_mode: AccessMode) {
    println!("access_mode: {:?}", access_mode);
    store::set_str_config(&app, store::ACCESS_MODE, access_mode.to_string().as_str()).unwrap();
    open_secc(app);
}

#[tauri::command]
pub fn switch_bind_mode(app: AppHandle, bind_mode: BindMode) {
    let app_handle = app.clone();
    println!("bind_mode: {:?}", bind_mode);
    match bind_mode {
        BindMode::Socks => {
            shell::switch_to_socks(app);
        }
        BindMode::Http => {
            shell::switch_to_http(app);
        }
    }
    store::set_str_config(
        &app_handle,
        store::BIND_MODE,
        bind_mode.to_string().as_str(),
    )
    .unwrap();
}

#[tauri::command]
pub fn switch_protocol_mode(app: AppHandle, protocol_mode: ProtocolMode) {
    let app_handle = app.clone();
    println!("protocol_mode: {:?}", protocol_mode);
    store::set_str_config(
        &app_handle,
        store::PROTOCOL_MODE,
        protocol_mode.to_string().as_str(),
    )
    .unwrap();
    open_secc(app_handle);
}

#[tauri::command]
pub fn get_access_mode(app: AppHandle) -> AccessMode {
    store::get_str_config(&app, store::ACCESS_MODE)
        .ok()
        .flatten()
        .and_then(|access_mode| access_mode.parse().ok())
        .unwrap_or(AccessMode::Auto)
}

#[tauri::command]
pub fn get_bind_mode(app: AppHandle) -> BindMode {
    store::get_str_config(&app, store::BIND_MODE)
        .ok()
        .flatten()
        .and_then(|bind_mode| bind_mode.parse().ok())
        .unwrap_or(BindMode::Socks)
}

#[tauri::command]
pub fn get_protocol_mode(app: AppHandle) -> ProtocolMode {
    store::get_str_config(&app, store::PROTOCOL_MODE)
        .ok()
        .flatten()
        .and_then(|protocol_mode| protocol_mode.parse().ok())
        .unwrap_or(ProtocolMode::Quic)
}

#[tauri::command]
pub fn get_servers(app: AppHandle) -> Option<Vec<ServerInfo>> {
    let result = store::get_servers(&app);
    result.unwrap_or_default()
}

#[tauri::command]
pub fn get_server(app: AppHandle, host: &str) -> Option<ServerInfo> {
    let result = store::get_server(&app, host);
    result.unwrap_or_default()
}

#[tauri::command]
pub fn add_server(app: AppHandle, server: ServerInfo) {
    store::add_server(&app, server).unwrap();
    app.emit("refresh_servers", ()).unwrap();
}

#[tauri::command]
pub fn delete_server(app: AppHandle, host: &str) {
    let cloned_app = app.clone();
    let res = store::get_str_config(&app, store::ACTIVE_SERVER);
    if let Ok(Some(server)) = res {
        if server == host {
            store::set_str_config(&cloned_app, store::ACTIVE_SERVER, "").unwrap();
            app.emit("active_server_disable", ()).unwrap();
            close_secc(app.clone());
        }
    }
    store::delete_server(&app, host).unwrap();
    app.emit("refresh_servers", ()).unwrap();
}

#[tauri::command]
pub fn update_server(app: AppHandle, server: ServerInfo) {
    store::update_server(&app, server).unwrap();
    app.emit("refresh_servers", ()).unwrap();
}

#[tauri::command]
pub fn active_server(app: AppHandle, host: &str) {
    store::set_str_config(&app, store::ACTIVE_SERVER, host).unwrap();
}

#[tauri::command]
pub fn get_active_server(app: AppHandle) -> Option<String> {
    let res = store::get_str_config(&app, store::ACTIVE_SERVER);
    res.ok().flatten()
}

#[tauri::command]
pub fn set_listen_config(app: AppHandle, listen_config: ListenConfig) {
    println!("request body: {:#?}", listen_config);
    let socks_config = AddrInfo::new(listen_config.socks_ip.clone(), listen_config.socks_port);
    let http_config = AddrInfo::new(listen_config.http_ip.clone(), listen_config.http_port);
    store::set_address(&app, SOCKS_ADDR, socks_config).unwrap();
    store::set_address(&app, HTTP_ADDR, http_config).unwrap();
}

#[tauri::command]
pub fn get_listen_config(app: AppHandle) -> ListenConfigOption {
    let result = store::get_address(&app, store::SOCKS_ADDR);
    let socks_config = result.unwrap_or_default();
    let result = store::get_address(&app, store::HTTP_ADDR);
    let http_config = result.unwrap_or_default();
    ListenConfigOption::new(socks_config, http_config)
}

#[tauri::command]
pub fn set_direct_rules(app: AppHandle, direct_rules: &str) {
    println!("request body: {:#?}", direct_rules);
    store::set_rules(&app, store::DIRECT_RULES_PATH, direct_rules).unwrap();
}

#[tauri::command]
pub fn get_direct_rules(app: AppHandle) -> String {
    let result = store::get_rules(&app, store::DIRECT_RULES_PATH);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn set_custom_proxy_rules(app: AppHandle, proxy_rules: &str, url: &str) {
    println!("request body: {:#?}, {}", proxy_rules, url);
    store::set_rules(&app, store::CUSTOM_PROXY_RULES_PATH, proxy_rules).unwrap();
    store::set_str_config(&app, store::COMMUNITY_RULES, url).unwrap();
    tauri::async_runtime::spawn(async move {
        store::load_community_proxy_list(&app).await.unwrap();
    });
}

#[tauri::command]
pub fn get_custom_proxy_rules(app: AppHandle) -> String {
    let result = store::get_rules(&app, store::CUSTOM_PROXY_RULES_PATH);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn get_combined_proxy_rules(app: AppHandle) -> String {
    let result = store::get_rules(&app, store::PROXY_RULES_PATH);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn set_cert(app: AppHandle, cert: &str) {
    println!("request body: {:#?}", cert);
    store::set_cert(&app, store::CERT_PATH, cert).unwrap();
}

#[tauri::command]
pub fn get_cert(app: AppHandle) -> String {
    let result = store::get_cert(&app, store::CERT_PATH);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn set_cert_key(app: AppHandle, cert_key: &str) {
    println!("request body: {:#?}", cert_key);
    store::set_cert(&app, store::CERT_KEY_PATH, cert_key).unwrap();
}

#[tauri::command]
pub fn get_cert_key(app: AppHandle) -> String {
    let result = store::get_cert(&app, store::CERT_KEY_PATH);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn close_app(app: AppHandle) {
    app.exit(0);
}
