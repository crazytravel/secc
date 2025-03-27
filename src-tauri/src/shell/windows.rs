use std::sync::Mutex;

use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_shell::ShellExt;
use tauri_plugin_shell::process::CommandEvent;

use crate::{
    state::{AccessMode, ProtocolMode, SidecarState},
    store::{self, CERT_PATH, DIRECT_RULES_PATH, PROXY_RULES_PATH, SERVER_ADDR},
    tray,
};

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

pub fn call_sidecar(app: &AppHandle, access_mode: AccessMode, protocol_mode: ProtocolMode) {
    println!("call sidecar function was called");
    let mut proxy_path = String::new();
    let mut direct_path = String::new();
    let mut cert_path = String::new();
    let mut server_addr = String::new();
    let proxy_list_res = store::get_path(app, PROXY_RULES_PATH);
    if let Ok(proxy_list) = proxy_list_res {
        if let Some(proxy_list_path) = proxy_list.to_str() {
            proxy_path = proxy_list_path.to_string();
        }
    }
    let direct_list_res = store::get_path(app, DIRECT_RULES_PATH);
    if let Ok(direct_list) = direct_list_res {
        if let Some(direct_list_path) = direct_list.to_str() {
            direct_path = direct_list_path.to_string();
        }
    }
    let cert_res = store::get_path(app, CERT_PATH);
    if let Ok(cert) = cert_res {
        if let Some(the_cert_path) = cert.to_str() {
            cert_path = the_cert_path.to_string();
        }
    }
    if let Ok(Some(config)) = store::get_address(app, SERVER_ADDR) {
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
            "-O",
            protocol_mode.to_string().as_str(),
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
        tray::change_tray_icon(app, true).unwrap();
    }
    sidecar_state.set(pid);
}
