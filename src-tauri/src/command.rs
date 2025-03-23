use std::{fmt, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Theme};
use tauri_plugin_shell::ShellExt;

use crate::menu::{self};

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

impl fmt::Display for AccessMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccessMode::Auto => write!(f, "AUTO"),
            AccessMode::Proxy => write!(f, "PROXY"),
            AccessMode::Direct => write!(f, "DIRECT"),
        }
    }
}

#[tauri::command]
pub fn call_sidecar(app: AppHandle, access_mode: AccessMode) {
    println!("call sidecar function was called");
    let sidecar_command = app.shell().sidecar("secc-agent").unwrap().args([
        "-p",
        "/Users/shuo/secc_config/proxy-list.txt",
        "-d",
        "/Users/shuo/secc_config/direct-list.txt",
        "-c",
        "/Users/shuo/secc_config/cert.pem",
        "-r",
        "185.212.58.6:443",
        "-a",
        &access_mode.to_string(),
    ]);
    let (_rx, child) = sidecar_command.spawn().unwrap();
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let mut sidecar_state = sidecar_state.lock().unwrap();
    let pid = child.pid();
    if pid != 0 {
        menu::change_tray_icon(&app, true).unwrap();
    }
    sidecar_state.set(pid);
}

#[tauri::command]
pub fn close_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
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

#[tauri::command]
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

#[tauri::command]
#[cfg(target_os = "macos")]
pub fn switch_to_direct(app: &AppHandle) {
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
