use std::{
    fmt::{self, Debug},
    str::FromStr,
    sync::Mutex,
};

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use sysinfo::{Pid, System};
use tauri::{AppHandle, Emitter, Manager, Theme};
use tauri_plugin_shell::{ShellExt, process::CommandEvent};

use crate::{
    menu::{self},
    server::{ListenConfig, ListenConfigOption, ServerConfig},
    store,
};

#[tauri::command]
pub fn open_secc(app: AppHandle) {
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let mut sidecar_state = sidecar_state.lock().unwrap();
    let pid = sidecar_state.get();
    let sys = System::new_all();
    let pid = Pid::from_u32(pid);
    if let Some(process) = sys.process(pid) {
        let status = process.kill();
        if !status {
            eprintln!("Kill sidecar process failed");
        }
    }
    let app_handle = app.clone();
    switch_to_socks(app_handle.clone());
    tauri::async_runtime::spawn(async move {
        call_sidecar(&app_handle, AccessMode::Auto);
    });
}

#[tauri::command]
pub fn close_secc(app: AppHandle) {
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let mut sidecar_state = sidecar_state.lock().unwrap();
    switch_to_direct(app.clone());
    let pid = sidecar_state.get();
    let sys = System::new_all();
    let pid = Pid::from_u32(pid);
    if let Some(process) = sys.process(pid) {
        let status = process.kill();
        if !status {
            eprintln!("Kill sidecar process failed");
            return;
        }
        menu::change_tray_icon(&app, false).unwrap();
    }
}

#[tauri::command]
pub fn switch_access_mode(app: AppHandle, access_mode: AccessMode) {
    println!("access_mode: {:?}", access_mode);
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let mut sidecar_state = sidecar_state.lock().unwrap();
    let pid = sidecar_state.get();
    let sys = System::new_all();
    let pid = Pid::from_u32(pid);
    if let Some(process) = sys.process(pid) {
        let status = process.kill();
        if !status {
            eprintln!("Kill sidecar process failed");
        }
    }
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        call_sidecar(&app_handle, access_mode);
    });
}

#[tauri::command]
pub fn switch_bind_mode(app: AppHandle, bind_mode: BindMode) {
    let app_handle = app.clone();
    println!("bind_mode: {:?}", bind_mode);
    match bind_mode {
        BindMode::Socks => {
            switch_to_socks(app);
        }
        BindMode::Http => {
            switch_to_http(app);
        }
    }
    store::set_bind_mode(&app_handle, bind_mode.to_string().as_str()).unwrap();
}

#[tauri::command]
pub fn get_access_mode(app: AppHandle) -> AccessMode {
    store::get_access_mode(&app)
        .ok()
        .flatten()
        .and_then(|access_mode| from_str(&access_mode).ok())
        .unwrap_or(AccessMode::Auto)
}

#[tauri::command]
pub fn get_bind_mode(app: AppHandle) -> BindMode {
    store::get_bind_mode(&app)
        .ok()
        .flatten()
        .and_then(|bind_mode| from_str(&bind_mode).ok())
        .unwrap_or(BindMode::Socks)
}

#[tauri::command]
pub fn set_server_config(app: AppHandle, server_config: ServerConfig) {
    println!("request body: {:#?}", server_config);
    store::set_server_config(&app, server_config).unwrap();
}

#[tauri::command]
pub fn get_server_config(app: AppHandle) -> Option<ServerConfig> {
    let result = store::get_server_config(&app);
    result.unwrap_or_default()
}

#[tauri::command]
pub fn set_listen_config(app: AppHandle, listen_config: ListenConfig) {
    println!("request body: {:#?}", listen_config);
    let socks_config = ServerConfig::new(listen_config.socks_ip.clone(), listen_config.socks_port);
    let http_config = ServerConfig::new(listen_config.http_ip.clone(), listen_config.http_port);
    store::set_socks_config(&app, socks_config).unwrap();
    store::set_http_config(&app, http_config).unwrap();
}

#[tauri::command]
pub fn get_listen_config(app: AppHandle) -> ListenConfigOption {
    let result = store::get_socks_config(&app);
    let socks_config = result.unwrap_or_default();
    let result = store::get_http_config(&app);
    let http_config = result.unwrap_or_default();
    ListenConfigOption::new(socks_config, http_config)
}

#[tauri::command]
pub fn set_direct_rules(app: AppHandle, direct_rules: &str) {
    println!("request body: {:#?}", direct_rules);
    store::set_direct_rules(&app, direct_rules).unwrap();
}

#[tauri::command]
pub fn get_direct_rules(app: AppHandle) -> String {
    let result = store::get_direct_rules(&app);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn set_proxy_rules(app: AppHandle, proxy_rules: &str) {
    println!("request body: {:#?}", proxy_rules);
    store::set_proxy_rules(&app, proxy_rules).unwrap();
}

#[tauri::command]
pub fn get_proxy_rules(app: AppHandle) -> String {
    let result = store::get_proxy_rules(&app);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn set_cert(app: AppHandle, cert: &str) {
    println!("request body: {:#?}", cert);
    store::set_cert(&app, cert).unwrap();
}

#[tauri::command]
pub fn get_cert(app: AppHandle) -> String {
    let result = store::get_cert(&app);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn set_cert_key(app: AppHandle, cert_key: &str) {
    println!("request body: {:#?}", cert_key);
    store::set_cert_key(&app, cert_key).unwrap();
}

#[tauri::command]
pub fn get_cert_key(app: AppHandle) -> String {
    let result = store::get_cert_key(&app);
    result.unwrap_or("".to_string())
}

#[tauri::command]
pub fn close_app(app: AppHandle) {
    app.exit(0);
}

#[cfg(target_os = "macos")]
pub fn switch_to_socks(app: AppHandle) {
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setautoproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxystate", "Wi-Fi", "on"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxy", "Wi-Fi", "127.0.0.1", "1080"])
        .spawn()
        .unwrap();

    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setautoproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxystate", "Ethernet", "on"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxy", "Ethernet", "127.0.0.1", "1080"])
        .spawn()
        .unwrap();
}

#[cfg(target_os = "macos")]
pub fn switch_to_http(app: AppHandle) {
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setautoproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxystate", "Wi-Fi", "on"])
        .spawn()
        .unwrap();

    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxy", "Wi-Fi", "127.0.0.1", "1081"])
        .spawn()
        .unwrap();

    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxystate", "Wi-Fi", "on"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxy", "Wi-Fi", "127.0.0.1", "1081"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();

    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setautoproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxystate", "Ethernet", "on"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxy", "Ethernet", "127.0.0.1", "1081"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxystate", "Ethernet", "on"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxy", "Ethernet", "127.0.0.1", "1081"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
}

#[cfg(target_os = "macos")]
pub fn switch_to_direct(app: AppHandle) {
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setautoproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxystate", "Wi-Fi", "off"])
        .spawn()
        .unwrap();

    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setautoproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsecurewebproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setwebproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
    let (_rx, _child) = app
        .shell()
        .command("networksetup")
        .args(["-setsocksfirewallproxystate", "Ethernet", "off"])
        .spawn()
        .unwrap();
}

pub fn call_sidecar(app: &AppHandle, access_mode: AccessMode) {
    println!("call sidecar function was called");
    let mut proxy_path = String::new();
    let mut direct_path = String::new();
    let mut cert_path = String::new();
    let mut server_addr = String::new();
    let proxy_list_res = store::get_proxy_list_path(app);
    if let Ok(proxy_list) = proxy_list_res {
        if let Some(proxy_list_path) = proxy_list.to_str() {
            proxy_path = proxy_list_path.to_string();
        }
    }
    let direct_list_res = store::get_direct_list_path(app);
    if let Ok(direct_list) = direct_list_res {
        if let Some(direct_list_path) = direct_list.to_str() {
            direct_path = direct_list_path.to_string();
        }
    }
    let cert_res = store::get_cert_path(app);
    if let Ok(cert) = cert_res {
        if let Some(the_cert_path) = cert.to_str() {
            cert_path = the_cert_path.to_string();
        }
    }
    if let Ok(Some(config)) = store::get_server_config(app) {
        server_addr = format!("{}:{}", config.host, config.port);
    }

    let sidecar_command = app
        .shell()
        .sidecar("secc-agent")
        .unwrap()
        .env("RUST_LOG", "INFO")
        .args([
            "-p",
            proxy_path.as_str(),
            "-d",
            direct_path.as_str(),
            "-c",
            cert_path.as_str(),
            "-r",
            server_addr.as_str(),
            "-a",
            access_mode.to_string().as_str(),
        ]);
    let (mut rx, child) = sidecar_command.spawn().unwrap();
    let app_handle = app.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    let log = String::from_utf8_lossy(&line);
                    app_handle.emit("secc-agent-log", log).unwrap();
                }
                CommandEvent::Stderr(line) => {
                    let log = String::from_utf8_lossy(&line);
                    app_handle.emit("secc-agent-log", log).unwrap();
                }
                CommandEvent::Error(error) => {
                    app_handle.emit("secc-agent-log", error).unwrap();
                }
                CommandEvent::Terminated(_) => {
                    app_handle.emit("secc-agent-log", "Terminated").unwrap();
                }
                _ => {}
            }
        }
    });
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let mut sidecar_state = sidecar_state.lock().unwrap();
    let pid = child.pid();
    if pid != 0 {
        menu::change_tray_icon(app, true).unwrap();
    }
    sidecar_state.set(pid);

    store::set_access_mode(app, access_mode.to_string().as_str()).unwrap();
}

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

#[derive(Debug)]
pub struct ThemeState(Option<Theme>);

impl ThemeState {
    pub fn default() -> Self {
        Self(None)
    }
    pub fn set(&mut self, theme: Theme) {
        self.0 = Some(theme);
    }
    pub fn get(&mut self) -> Option<Theme> {
        self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccessMode {
    Auto,
    Proxy,
    Direct,
}

impl FromStr for AccessMode {
    type Err = String; // Or a more specific error type if you prefer

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "AUTO" => Ok(AccessMode::Auto),
            "PROXY" => Ok(AccessMode::Proxy),
            "DIRECT" => Ok(AccessMode::Direct),
            _ => Err(format!("Invalid access mode: {}", s)),
        }
    }
}

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessMode::Auto => write!(f, "AUTO"),
            AccessMode::Proxy => write!(f, "PROXY"),
            AccessMode::Direct => write!(f, "DIRECT"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum BindMode {
    Socks,
    Http,
}

impl FromStr for BindMode {
    type Err = String; // Or a more specific error type if you prefer

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "SOCKS" => Ok(BindMode::Socks),
            "HTTP" => Ok(BindMode::Http),
            _ => Err(format!("Invalid access mode: {}", s)),
        }
    }
}

impl fmt::Display for BindMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BindMode::Socks => write!(f, "SOCKS"),
            BindMode::Http => write!(f, "HTTP"),
        }
    }
}
