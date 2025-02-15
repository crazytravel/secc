use tauri::{
    image::Image,
    menu::{CheckMenuItem, MenuBuilder, MenuItem, SubmenuBuilder},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

fn open_main_window(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    {
        tauri::AppHandle::show(app).unwrap();
    }
}

#[tauri::command]
fn close_app(app: AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![close_app])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);
            }
            let icon_bytes = include_bytes!("../icons/tray-icon.png");
            let setting = MenuItem::with_id(app, "setting", "Setting", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;

            let auto_model =
                CheckMenuItem::with_id(app, "auto_model", "Auto Model", true, false, None::<&str>)?;
            let proxy_model = CheckMenuItem::with_id(
                app,
                "proxy_model",
                "Proxy Model",
                true,
                false,
                None::<&str>,
            )?;
            let direct_model = CheckMenuItem::with_id(
                app,
                "direct_model",
                "Direct Model",
                true,
                false,
                None::<&str>,
            )?;
            let node = MenuItem::with_id(app, "node", "Node", true, None::<&str>)?;
            let server = SubmenuBuilder::new(app, "Servers").item(&node).build()?;

            let menu = MenuBuilder::new(app)
                .item(&auto_model)
                .item(&proxy_model)
                .item(&direct_model)
                .separator()
                .item(&server)
                .separator()
                .item(&setting)
                .separator()
                .item(&quit)
                .build()?;
            let _tray = TrayIconBuilder::new()
                .tooltip("Secure Connect")
                .icon_as_template(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "setting" => {
                        println!("setting menu item was clicked");
                        open_main_window(app);
                    }
                    "quit" => {
                        println!("quit menu item was clicked");
                        app.exit(0);
                    }
                    _ => {
                        println!("menu item {:?} not handled", event.id);
                    }
                })
                .menu(&menu)
                .icon(Image::from_bytes(icon_bytes)?)
                .build(app)?;

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
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
