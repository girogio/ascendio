mod errors;
mod functions;
mod models;

use functions::serial::start_checking_for_device;
use std::sync::Mutex;
use tauri::Manager;

use crate::{
    functions::{serial, sim_event},
    models::AppContext,
};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{} {}] {}",
                        record.level(),
                        record.target(),
                        message
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_serialplugin::init())
        .invoke_handler(tauri::generate_handler![
            serial::get_serial_ports,
            serial::is_connected,
            serial::try_connect,
            serial::disconnect,
            sim_event::send_event
        ])
        .setup(|app| {
            let ctx = AppContext::new();
            app.manage(Mutex::new(ctx));
            start_checking_for_device(app.handle().clone());
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
