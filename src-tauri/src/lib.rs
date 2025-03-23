use std::sync::Mutex;

use anyhow::Error;
use command::{SidecarState, ThemeState};
use sysinfo::{Pid, System};
use tauri::{App, AppHandle, Manager, Theme};
mod command;
mod menu;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SidecarState::default()))
        .manage(Mutex::new(ThemeState::default()))
        .invoke_handler(tauri::generate_handler![
            command::close_app,
            command::call_sidecar
        ])
        .setup(|app| {
            init_setup(app)?;
            Ok(())
        })
        .on_window_event(|win, event| {
            let app = win.app_handle();
            if let tauri::WindowEvent::ThemeChanged(theme) = event {
                if *theme == Theme::Dark {
                    app.state::<Mutex<ThemeState>>()
                        .lock()
                        .unwrap()
                        .set(Theme::Dark);
                } else {
                    app.state::<Mutex<ThemeState>>()
                        .lock()
                        .unwrap()
                        .set(Theme::Light);
                }
                let pid = app.state::<Mutex<SidecarState>>().lock().unwrap().get();
                let sys = System::new_all();
                let pid = Pid::from_u32(pid);
                println!("pid: {}", pid);
                if sys.process(pid).is_some() {
                    println!("process exist");
                    menu::change_tray_icon(app, true).unwrap();
                } else {
                    println!("process not exist");
                    menu::change_tray_icon(app, false).unwrap();
                }
            }

            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                #[cfg(not(target_os = "macos"))]
                {
                    event.window().hide().unwrap();
                }
                #[cfg(target_os = "macos")]
                {
                    tauri::AppHandle::hide(win.app_handle()).unwrap();
                }
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app_handle, event| {
            if let tauri::RunEvent::Exit { .. } = event {
                // clean up things
                println!("---clean up start---");
                let sidecar_state = app_handle.state::<Mutex<SidecarState>>();
                let mut sidecar_state = sidecar_state.lock().unwrap();
                command::switch_to_direct(app_handle);
                let pid = sidecar_state.get();
                let sys = System::new_all();
                let pid = Pid::from_u32(pid);
                if let Some(process) = sys.process(pid) {
                    let status = process.kill();
                    if !status {
                        eprintln!("Kill sidecar process failed");
                    }
                }
                println!("---clean up end---");
            }
        });
}

fn init_setup(app: &mut App) -> Result<(), Error> {
    #[cfg(target_os = "macos")]
    {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
    }
    // add tray menu
    menu::build_menu(app.handle())?;
    auto_start(app.handle().clone());
    Ok(())
}

fn auto_start(app: AppHandle) {
    let cloned_app = app.clone();
    // Start the sidecar when the app starts
    tauri::async_runtime::spawn(async move {
        command::call_sidecar(app, command::AccessMode::Auto);
    });
    // Auto set proxy address
    tauri::async_runtime::spawn(async move {
        command::switch_to_socks(cloned_app);
    });
}
