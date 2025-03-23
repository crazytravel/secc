use std::sync::Mutex;

use crate::command::{self, SidecarState, ThemeState};
use anyhow::{Error, Ok};
use sysinfo::{Pid, System};
use tauri::menu::PredefinedMenuItem;
use tauri::{
    AppHandle,
    image::Image,
    menu::{CheckMenuItem, MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
};
use tauri::{Manager, Theme};

pub const APP_TRAY_ID: &str = "secc-tray";

pub fn change_tray_icon(app: &AppHandle, active: bool) -> Result<(), Error> {
    let theme = { app.state::<Mutex<ThemeState>>().lock().unwrap().get() };
    println!("==theme: {:?}", theme);
    let icon_bytes = {
        match (active, theme) {
            (true, Some(tauri::Theme::Dark)) => {
                println!("active & dark");
                Some(include_bytes!("../icons/tray-icon-dark-active.png").to_vec())
            }
            (false, Some(tauri::Theme::Dark)) => {
                println!("inactive & dark");
                Some(include_bytes!("../icons/tray-icon-dark-inactive.png").to_vec())
            }
            (true, Some(tauri::Theme::Light)) => {
                println!("active & light");
                Some(include_bytes!("../icons/tray-icon-light-active.png").to_vec())
            }
            (false, Some(tauri::Theme::Light)) => {
                println!("inactive & light");
                Some(include_bytes!("../icons/tray-icon-light-inactive.png").to_vec())
            }
            _ => None,
        }
    };
    if let Some(tray) = app.tray_by_id(APP_TRAY_ID) {
        if let Some(icon_bytes) = icon_bytes {
            tray.set_icon(Some(Image::from_bytes(&icon_bytes)?))?;
        }
    }
    Ok(())
}

pub fn build_menu(app: &AppHandle) -> Result<(), Error> {
    let theme = app.get_webview_window("main").unwrap().theme().unwrap();
    println!("theme: {:?}", theme);
    let icon_bytes = if theme == Theme::Dark {
        include_bytes!("../icons/tray-icon-dark-active.png").to_vec()
    } else {
        include_bytes!("../icons/tray-icon-light-active.png").to_vec()
    };
    let setting = MenuItem::with_id(app, "setting", "Setting", true, None::<&str>)?;
    let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;
    let auto_model =
        CheckMenuItem::with_id(app, "auto_model", "Auto Model", false, true, None::<&str>)?;
    let proxy_model = CheckMenuItem::with_id(
        app,
        "proxy_model",
        "Global Model",
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
    let socks_model =
        CheckMenuItem::with_id(app, "socks_model", "Socks", false, true, None::<&str>)?;
    let http_model = CheckMenuItem::with_id(app, "http_model", "Http", true, false, None::<&str>)?;
    // let node = MenuItem::with_id(app, "node", "Node", true, None::<&str>)?;
    // let server = SubmenuBuilder::new(app, "Servers").item(&node).build()?;
    let menu = MenuBuilder::new(app)
        .item(&auto_model)
        .item(&proxy_model)
        .item(&direct_model)
        .separator()
        .item(&socks_model)
        .item(&http_model)
        .separator()
        // .item(&server)
        // .separator()
        .item(&setting)
        // .separator()
        .item(&quit)
        .build()?;
    TrayIconBuilder::with_id(APP_TRAY_ID)
        .tooltip("Secure Connect")
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "auto_model" => {
                println!("auto model menu item was clicked");
                toggle_model(&auto_model, &proxy_model, &direct_model, "auto_model");
                let app_side_handle = app.clone();
                let app_handle = app.clone();
                {
                    let sidecar_state = app_handle.state::<Mutex<SidecarState>>();
                    let pid = sidecar_state.lock().unwrap().get();
                    let sys = System::new_all();
                    let pid = Pid::from_u32(pid);
                    if let Some(process) = sys.process(pid) {
                        let status = process.kill();
                        if !status {
                            eprintln!("Kill sidecar process failed");
                        }
                    }
                }

                tauri::async_runtime::spawn(async move {
                    command::call_sidecar(app_side_handle, command::AccessMode::Auto);
                });
            }
            "proxy_model" => {
                println!("socks model menu item was clicked");
                toggle_model(&auto_model, &proxy_model, &direct_model, "proxy_model");

                let app_side_handle = app.clone();
                let app_handle = app.clone();
                {
                    let sidecar_state = app_handle.state::<Mutex<SidecarState>>();
                    let pid = sidecar_state.lock().unwrap().get();
                    let sys = System::new_all();
                    let pid = Pid::from_u32(pid);
                    if let Some(process) = sys.process(pid) {
                        let status = process.kill();
                        if !status {
                            eprintln!("Kill sidecar process failed");
                        }
                    }
                }
                tauri::async_runtime::spawn(async move {
                    command::call_sidecar(app_side_handle, command::AccessMode::Proxy);
                });
            }
            "direct_model" => {
                println!("direct model menu item was clicked");
                toggle_model(&auto_model, &proxy_model, &direct_model, "direct_model");
                command::switch_to_direct(app);

                let app_handle = app.clone();
                {
                    let sidecar_state = app_handle.state::<Mutex<SidecarState>>();
                    let pid = sidecar_state.lock().unwrap().get();
                    let sys = System::new_all();
                    let pid = Pid::from_u32(pid);
                    if let Some(process) = sys.process(pid) {
                        let status = process.kill();
                        if !status {
                            eprintln!("Kill sidecar process failed");
                        }
                    }
                }
            }
            "socks_model" => {
                println!("socks proxy model menu item was clicked");
                toggle_protocol(&socks_model, &http_model, "socks_model");
                command::switch_to_socks(app.clone());
            }
            "http_model" => {
                println!("http proxy model menu item was clicked");
                toggle_protocol(&socks_model, &http_model, "http_model");
                command::switch_to_http(app.clone());
            }
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
        .icon(Image::from_bytes(&icon_bytes)?)
        .build(app)?;

    Ok(())
}

fn open_main_window(app: &AppHandle) {
    #[cfg(target_os = "macos")]
    {
        tauri::AppHandle::show(app).unwrap();
    }
}

fn toggle_protocol<R: tauri::Runtime>(
    socks_model: &CheckMenuItem<R>,
    http_model: &CheckMenuItem<R>,
    selected_id: &str,
) {
    match selected_id {
        "socks_model" => {
            socks_model.set_checked(true).unwrap();
            socks_model.set_enabled(false).unwrap();
            http_model.set_checked(false).unwrap();
            http_model.set_enabled(true).unwrap();
        }
        "http_model" => {
            http_model.set_checked(true).unwrap();
            http_model.set_enabled(false).unwrap();
            socks_model.set_checked(false).unwrap();
            socks_model.set_enabled(true).unwrap();
        }
        _ => {}
    }
}

fn toggle_model<R: tauri::Runtime>(
    auto_model: &CheckMenuItem<R>,
    proxy_model: &CheckMenuItem<R>,
    direct_model: &CheckMenuItem<R>,
    selected_id: &str,
) {
    match selected_id {
        "auto_model" => {
            auto_model.set_checked(true).unwrap();
            auto_model.set_enabled(false).unwrap();
            proxy_model.set_checked(false).unwrap();
            proxy_model.set_enabled(true).unwrap();
            direct_model.set_checked(false).unwrap();
            direct_model.set_enabled(true).unwrap();
        }
        "proxy_model" => {
            proxy_model.set_checked(true).unwrap();
            proxy_model.set_enabled(false).unwrap();
            auto_model.set_checked(false).unwrap();
            auto_model.set_enabled(true).unwrap();
            direct_model.set_checked(false).unwrap();
            direct_model.set_enabled(true).unwrap();
        }
        "direct_model" => {
            direct_model.set_checked(true).unwrap();
            direct_model.set_enabled(false).unwrap();
            auto_model.set_checked(false).unwrap();
            auto_model.set_enabled(true).unwrap();
            proxy_model.set_checked(false).unwrap();
            proxy_model.set_enabled(true).unwrap();
        }
        _ => {}
    }
}
