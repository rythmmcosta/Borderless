#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;

use std::sync::Arc;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager,
};
use tokio::sync::Mutex;
use borderless_core::{BorderlessEngine, EngineConfig};

pub struct AppState {
    pub engine: Arc<Mutex<Option<BorderlessEngine>>>,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    tauri::Builder::default()
        .setup(|app| {
            let quit  = MenuItem::with_id(app, "quit",  "Quit",          true, None::<&str>)?;
            let show  = MenuItem::with_id(app, "show",  "Show Window",   true, None::<&str>)?;
            let about = MenuItem::with_id(app, "about", "About Borderless", true, None::<&str>)?;
            let menu  = Menu::with_items(app, &[&show, &about, &quit])?;

            TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Borderless — KVM Sync")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "quit" => app.exit(0),
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    _ => {}
                })
                .build(app)?;

            Ok(())
        })
        .manage(AppState {
            engine: Arc::new(Mutex::new(None)),
        })
        .invoke_handler(tauri::generate_handler![
            commands::start_engine,
            commands::stop_engine,
            commands::list_devices,
            commands::connect_device,
            commands::disconnect_device,
            commands::engine_status,
        ])
        .run(tauri::generate_context!())
        .expect("failed to run Borderless");
}
