use std::sync::Mutex;

use command::SidecarState;
use tauri::{AppHandle, Manager};
mod command;
mod menu;

fn auto_start(app: AppHandle) {
    let cloned_app = app.clone();
    // Start the sidecar when the app starts
    tauri::async_runtime::spawn(async move {
        command::call_sidecar(app).await;
    });

    // Auto set proxy address
    command::switch_to_socks(cloned_app);
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SidecarState::default()))
        .invoke_handler(tauri::generate_handler![
            command::close_app,
            command::call_sidecar
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            // add tray menu
            menu::build_menu(app.handle())?;
            // auto start process
            auto_start(app.handle().clone());
            // 
            
            Ok(())
        })
        .on_window_event(|win, event| {
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
                sidecar_state.get().unwrap().kill().expect("kill sidecar process failed");
                println!("---clean up end---");
            }
        });
}
