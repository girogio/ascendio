use log::info;
use serialport::SerialPortInfo;
use std::sync::Mutex;

use tauri::{Emitter, Manager};

use crate::errors::Result;

type AppContext<'a> = tauri::State<'a, Mutex<crate::models::AppContext>>;

/// Returns an array of objects representing available USB serial ports.
///
/// # Returns
///
/// A `Result` containing a vector of `SerialPortInfo` objects. Each object has the following JSON structure:
///
/// ```json
/// {
///   "port_name": "string",
///   "port_type": {
///     "UsbPort": {
///       "vid": "number",
///       "pid": "number",
///       "serial_number": "string",
///       "manufacturer": "string",
///       "product": "string"
///     }
///   }
/// }
/// ```
///
/// # Errors
///
/// Returns an error if there is an issue retrieving the available serial ports.
#[tauri::command]
pub fn get_serial_ports() -> Result<Vec<SerialPortInfo>> {
    let ports = serialport::available_ports()?
        .iter()
        .filter(|p| matches!(p.port_type, serialport::SerialPortType::UsbPort(_)))
        .cloned()
        .collect();

    Ok(ports)
}

#[tauri::command]
pub async fn try_connect(state: AppContext<'_>) -> Result<()> {
    let mut state = state.lock().unwrap();
    let baud_rate = state.config.baud_rate;

    state.mcu.try_connect(baud_rate)?;

    Ok(())
}

#[tauri::command]
pub async fn disconnect(state: AppContext<'_>) -> Result<()> {
    let mut state = state.lock().unwrap();

    state.mcu.disconnect()?;

    Ok(())
}

/// Returns a boolean indicating whether the MCU is connected.
#[tauri::command]
pub async fn is_connected(state: AppContext<'_>) -> Result<bool> {
    let mut state = state.lock().unwrap();

    let baud_rate = state.config.baud_rate;
    state.mcu.try_connect(baud_rate)?;

    Ok(state.mcu.is_connected())
}

pub fn start_checking_for_device(window: tauri::AppHandle) {
    std::thread::spawn(move || loop {
        let state: AppContext = window.state();

        let mut state = state.lock().unwrap();

        if state.mcu.is_connected() {
            window
                .emit("mcu-connected", ())
                .expect("failed to emit deviceConnected event");
            info!("MCU connected");
        } else {
            window
                .emit("mcu-disconnected", ())
                .expect("failed to emit deviceDisconnected event");
            info!("MCU disconnected");
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    });
}
