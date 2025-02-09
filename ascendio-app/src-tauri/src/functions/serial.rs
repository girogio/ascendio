use serialport::SerialPortInfo;
use tauri::async_runtime::Mutex;

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

/// Returns a boolean indicating whether the MCU is connected.
#[tauri::command]
pub async fn is_connected(state: AppContext<'_>) -> Result<bool> {
    let mut state = state.lock().await;

    let baud_rate = state.config.baud_rate;
    state.mcu.try_connect(baud_rate)?;

    Ok(state.mcu.is_connected())
}
