use crate::command::{self};
use anyhow::{Error, Ok};
use tauri::menu::PredefinedMenuItem;
use tauri::{
    AppHandle,
    image::Image,
    menu::{CheckMenuItem, MenuBuilder, MenuItem, SubmenuBuilder},
    tray::TrayIconBuilder,
};

pub fn build_menu(app: &AppHandle) -> Result<(), Error> {
    let icon_bytes = include_bytes!("../icons/tray-icon.png");
    let setting = MenuItem::with_id(app, "setting", "Setting", true, None::<&str>)?;
    let quit = PredefinedMenuItem::quit(app, Some("Quit"))?;
    let vpn_model =
        CheckMenuItem::with_id(app, "vpn_model", "VPN Model", true, false, None::<&str>)?;
    let proxy_model =
        CheckMenuItem::with_id(app, "proxy_model", "Proxy Model", false, true, None::<&str>)?;
    let auto_model = CheckMenuItem::with_id(app, "auto_model", "Auto", true, false, None::<&str>)?;
    let socks_proxy_model =
        CheckMenuItem::with_id(app, "socks_proxy_model", "Socks", false, true, None::<&str>)?;
    let http_proxy_model =
        CheckMenuItem::with_id(app, "http_proxy_model", "Http", true, false, None::<&str>)?;
    let direct_model =
        CheckMenuItem::with_id(app, "direct_model", "Direct", true, false, None::<&str>)?;
    let node = MenuItem::with_id(app, "node", "Node", true, None::<&str>)?;
    let server = SubmenuBuilder::new(app, "Servers").item(&node).build()?;
    let menu = MenuBuilder::new(app)
        .item(&proxy_model)
        .item(&vpn_model)
        .separator()
        .item(&auto_model)
        .item(&socks_proxy_model)
        .item(&http_proxy_model)
        .item(&direct_model)
        .separator()
        .item(&server)
        .separator()
        .item(&setting)
        .separator()
        .item(&quit)
        .build()?;
    TrayIconBuilder::new()
        .tooltip("Secure Connect")
        .icon_as_template(true)
        .on_menu_event(move |app, event| match event.id.as_ref() {
            "vpn_model" => {
                println!("vpn model menu item was clicked");
                toggle_protocol(&vpn_model, &proxy_model, "vpn_model");
            }
            "proxy_model" => {
                println!("socks model menu item was clicked");
                toggle_protocol(&vpn_model, &proxy_model, "proxy_model");
            }
            "auto_model" => {
                println!("auto model menu item was clicked");
                toggle_model(
                    &auto_model,
                    &socks_proxy_model,
                    &http_proxy_model,
                    &direct_model,
                    "auto_model",
                );
            }
            "socks_proxy_model" => {
                println!("socks proxy model menu item was clicked");
                toggle_model(
                    &auto_model,
                    &socks_proxy_model,
                    &http_proxy_model,
                    &direct_model,
                    "socks_proxy_model",
                );
                command::switch_to_socks(app.clone());
            }
            "http_proxy_model" => {
                println!("http proxy model menu item was clicked");
                toggle_model(
                    &auto_model,
                    &socks_proxy_model,
                    &http_proxy_model,
                    &direct_model,
                    "http_proxy_model",
                );
                command::switch_to_http(app.clone());
            }
            "direct_model" => {
                println!("direct model menu item was clicked");
                toggle_model(
                    &auto_model,
                    &socks_proxy_model,
                    &http_proxy_model,
                    &direct_model,
                    "direct_model",
                );
                command::switch_to_direct(app);
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
        .icon(Image::from_bytes(icon_bytes)?)
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
    vpn_model: &CheckMenuItem<R>,
    proxy_model: &CheckMenuItem<R>,
    selected_id: &str,
) {
    match selected_id {
        "vpn_model" => {
            vpn_model.set_checked(true).unwrap();
            vpn_model.set_enabled(false).unwrap();
            proxy_model.set_checked(false).unwrap();
            proxy_model.set_enabled(true).unwrap();
        }
        "proxy_model" => {
            proxy_model.set_checked(true).unwrap();
            proxy_model.set_enabled(false).unwrap();
            vpn_model.set_checked(false).unwrap();
            vpn_model.set_enabled(true).unwrap();
        }
        _ => {}
    }
}

fn toggle_model<R: tauri::Runtime>(
    auto_model: &CheckMenuItem<R>,
    socks_proxy_model: &CheckMenuItem<R>,
    http_proxy_model: &CheckMenuItem<R>,
    direct_model: &CheckMenuItem<R>,
    selected_id: &str,
) {
    match selected_id {
        "auto_model" => {
            auto_model.set_checked(true).unwrap();
            auto_model.set_enabled(false).unwrap();
            socks_proxy_model.set_checked(false).unwrap();
            socks_proxy_model.set_enabled(true).unwrap();
            http_proxy_model.set_checked(false).unwrap();
            http_proxy_model.set_enabled(true).unwrap();
            direct_model.set_checked(false).unwrap();
            direct_model.set_enabled(true).unwrap();
        }
        "socks_proxy_model" => {
            socks_proxy_model.set_checked(true).unwrap();
            socks_proxy_model.set_enabled(false).unwrap();
            auto_model.set_checked(false).unwrap();
            auto_model.set_enabled(true).unwrap();
            http_proxy_model.set_checked(false).unwrap();
            http_proxy_model.set_enabled(true).unwrap();
            direct_model.set_checked(false).unwrap();
            direct_model.set_enabled(true).unwrap();
        }
        "direct_model" => {
            direct_model.set_checked(true).unwrap();
            direct_model.set_enabled(false).unwrap();
            auto_model.set_checked(false).unwrap();
            auto_model.set_enabled(true).unwrap();
            socks_proxy_model.set_checked(false).unwrap();
            socks_proxy_model.set_enabled(true).unwrap();
            http_proxy_model.set_checked(false).unwrap();
            http_proxy_model.set_enabled(true).unwrap();
        }
        "http_proxy_model" => {
            http_proxy_model.set_checked(true).unwrap();
            http_proxy_model.set_enabled(false).unwrap();
            auto_model.set_checked(false).unwrap();
            auto_model.set_enabled(true).unwrap();
            socks_proxy_model.set_checked(false).unwrap();
            socks_proxy_model.set_enabled(true).unwrap();
            direct_model.set_checked(false).unwrap();
            direct_model.set_enabled(true).unwrap();
        }
        _ => {}
    }
}
