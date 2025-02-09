mod errors;
mod functions;
mod models;

use tauri::{async_runtime::Mutex, Manager};

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
            sim_event::send_event
        ])
        .setup(|app| {
            let app_context = Mutex::new(AppContext::new());
            app.manage(app_context);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
