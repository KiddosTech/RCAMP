use rcamp_core::{execute_shell, flash_firmware, ConnectionState, Device, DeviceManager, DeviceProfile, FlashRequest, FlashTarget, TransportKind};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

struct AppState { devices: Mutex<DeviceManager>, logs: Mutex<Vec<LogEntry>> }

#[derive(Serialize, Clone)]
struct LogEntry { timestamp: u64, level: String, message: String }

fn record_log(state: &AppState, level: &str, message: impl Into<String>) {
    if let Ok(mut logs) = state.logs.lock() {
        logs.push(LogEntry { timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_secs()), level: level.into(), message: message.into() });
        if logs.len() > 500 { let _ = logs.drain(..logs.len() - 500); }
    }
}

#[derive(Serialize)]
struct DeviceView {
    name: String,
    device_type: String,
    transport: TransportKind,
    address: String,
    state: ConnectionState,
}

#[tauri::command]
fn list_devices(state: State<'_, AppState>) -> Result<Vec<DeviceView>, String> {
    let manager = state.devices.lock().map_err(|_| "device manager unavailable")?;
    Ok(manager
        .devices()
        .into_iter()
        .map(|device: &Device| DeviceView {
            name: device.name.clone(),
            device_type: device.device_type.clone(),
            transport: device.transport,
            address: device.address.clone(),
            state: device.state,
        })
        .collect())
}

#[tauri::command]
fn add_device(profile: DeviceProfile, state: State<'_, AppState>) -> Result<DeviceView, String> {
    profile.validate().map_err(|error| error.to_string())?;
    let device = profile.to_device();
    let view = DeviceView {
        name: device.name.clone(),
        device_type: device.device_type.clone(),
        transport: device.transport,
        address: device.address.clone(),
        state: device.state,
    };
    let mut manager = state.devices.lock().map_err(|_| "device manager unavailable")?;
    manager.register(device);
    record_log(&state, "info", format!("registered device {}", profile.name));
    Ok(view)
}

#[tauri::command]
fn list_logs(state: State<'_, AppState>) -> Result<Vec<LogEntry>, String> {
    Ok(state.logs.lock().map_err(|_| "log store unavailable")?.clone())
}

#[tauri::command]
fn shell_command(address: String, command: String, state: State<'_, AppState>) -> Result<String, String> {
    record_log(&state, "info", format!("shell command sent to {address}: {command}"));
    match execute_shell(&address, &command) {
        Ok(response) => {
            record_log(&state, "info", format!("shell response received in {}ms", response.elapsed_ms));
            if command.trim().eq_ignore_ascii_case("rcampfetch") {
                let host = std::env::var("COMPUTERNAME").or_else(|_| std::env::var("HOSTNAME")).unwrap_or_else(|_| "unknown".into());
                let cpus = std::thread::available_parallelism().map_or(0, |value| value.get());
                Ok(format!("RCAMPFETCH\nhost={host}\nos={}\narch={}\ncpus={cpus}\n\n{}", std::env::consts::OS, std::env::consts::ARCH, response.output))
            } else { Ok(response.output) }
        }
        Err(error) => { record_log(&state, "error", error.to_string()); Err(error.to_string()) }
    }
}

#[tauri::command]
fn flash_target(target: String, port: String, firmware: String) -> Result<String, String> {
    let target = match target.to_ascii_lowercase().as_str() {
        "esp32" => FlashTarget::Esp32,
        "arduino" => FlashTarget::Arduino,
        _ => return Err("target must be esp32 or arduino".into()),
    };
    flash_firmware(&FlashRequest { target, port, firmware: firmware.into() }).map_err(|error| error.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .manage(AppState { devices: Mutex::new(DeviceManager::new()), logs: Mutex::new(Vec::new()) })
        .invoke_handler(tauri::generate_handler![list_devices, add_device, flash_target, list_logs, shell_command])
        .run(tauri::generate_context!())
        .expect("error while running RCAMP");
}
