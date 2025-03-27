use std::sync::Mutex;

use anyhow::Error;
use state::SidecarState;
use tauri::{App, Manager};
mod command;
mod server;
mod shell;
mod state;
mod store;
mod tray;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(SidecarState::default()))
        .invoke_handler(tauri::generate_handler![
            command::close_app,
            command::set_server_config,
            command::get_server_config,
            command::set_listen_config,
            command::get_listen_config,
            command::set_direct_rules,
            command::get_direct_rules,
            command::set_proxy_rules,
            command::get_proxy_rules,
            command::set_cert,
            command::get_cert,
            command::set_cert_key,
            command::get_cert_key,
            command::switch_bind_mode,
            command::open_secc,
            command::close_secc,
            command::switch_access_mode,
            command::get_access_mode,
            command::get_bind_mode,
            command::switch_protocol_mode,
            command::get_protocol_mode,
        ])
        .setup(|app| {
            let app_handle = app.app_handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = store::init_all(&app_handle).await {
                    eprintln!("initial config files error: {:?}", e);
                }
            });
            init_setup(app)?;
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
                command::close_secc(app_handle.clone());
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
    tray::build_tray(app.handle())?;
    command::open_secc(app.handle().clone());
    Ok(())
}
