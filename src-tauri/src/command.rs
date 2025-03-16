use std::sync::Mutex;

use tauri::{AppHandle, Manager};
use tauri_plugin_shell::{process::CommandChild, ShellExt};

#[derive(Debug)]
pub struct SidecarState(Option<CommandChild>);

impl SidecarState {
    pub fn default() -> Self {
        Self(None)
    }
    pub fn set(&mut self, child: CommandChild) {
        self.0 = Some(child);
    }
    pub fn get(&mut self) -> Option<CommandChild> {
        self.0.take()
    }
}

#[tauri::command]
pub async fn call_sidecar(app: AppHandle) {
    let sidecar_command = app.shell().sidecar("secc-socks").unwrap().args(["no-auth"]);
    let (_rx, child) = sidecar_command.spawn().unwrap();
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let mut sidecar_state = sidecar_state.lock().unwrap();
    sidecar_state.set(child);
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
