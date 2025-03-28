#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

use std::sync::Mutex;

use crate::state::{AccessMode, ProtocolMode, SidecarState};
use anyhow::{Error, anyhow};
use sysinfo::{Pid, System};
use tauri::{AppHandle, Manager};

pub fn switch_to_socks(app: AppHandle) {
    #[cfg(target_os = "macos")]
    macos::switch_to_socks(app);
    #[cfg(target_os = "linux")]
    linux::switch_to_socks(app);
    #[cfg(target_os = "windows")]
    windows::switch_to_socks(app);
}
pub fn switch_to_http(app: AppHandle) {
    #[cfg(target_os = "macos")]
    macos::switch_to_http(app);
    #[cfg(target_os = "linux")]
    linux::switch_to_http(app);
    #[cfg(target_os = "windows")]
    windows::switch_to_http(app);
}
pub fn switch_to_direct(app: AppHandle) {
    #[cfg(target_os = "macos")]
    macos::switch_to_direct(app);
    #[cfg(target_os = "linux")]
    linux::switch_to_direct(app);
    #[cfg(target_os = "windows")]
    windows::switch_to_direct(app);
}
pub fn call_sidecar(app: &AppHandle, access_mode: AccessMode, protocol_mode: ProtocolMode) {
    #[cfg(target_os = "macos")]
    macos::call_sidecar(app, access_mode, protocol_mode);
    #[cfg(target_os = "linux")]
    linux::call_sidecar(app, access_mode, protocol_mode);
    #[cfg(target_os = "windows")]
    windows::call_sidecar(app, access_mode, protocol_mode);
}

pub fn kill_sidecar(app: &AppHandle) -> Result<(), Error> {
    let sidecar_state = app.state::<Mutex<SidecarState>>();
    let sidecar_state = sidecar_state.lock().unwrap();
    let pid = sidecar_state.get();
    let sys = System::new_all();
    let pid = Pid::from_u32(pid);
    if let Some(process) = sys.process(pid) {
        let status = process.kill();
        if !status {
            return Err(anyhow!("Kill sidecar process failed"));
        }
    }
    Ok(())
}
