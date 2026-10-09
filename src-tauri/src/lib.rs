use rcamp_core::{execute_shell, flash_firmware, ConnectionState, Device, DeviceManager, DeviceProfile, FlashRequest, FlashTarget, TransportKind};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

struct AppState { devices: Mutex<DeviceManager>, logs: Mutex<Vec<LogEntry>> }

#[derive(Serialize, Clone)]
struct LogEntry { timestamp: u64, level: String, message: String }

#[derive(Serialize)]
struct SerialCandidate {
    port: String,
    kind: String,
    identity: String,
    classification: String,
    safe_to_flash: bool,
}

fn record_log(state: &AppState, level: &str, message: impl Into<String>) {
    if let Ok(mut logs) = state.logs.lock() {
        logs.push(LogEntry { timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_secs()), level: level.into(), message: message.into() });
        let excess = logs.len().saturating_sub(500);
        if excess > 0 { let _ = logs.drain(..excess); }
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
fn search_serial_devices() -> Result<Vec<SerialCandidate>, String> {
    #[cfg(target_os = "android")]
    { return Ok(Vec::new()); }

    #[cfg(not(target_os = "android"))]
    {
        let ports = serialport::available_ports().map_err(|error| error.to_string())?;
        Ok(ports.into_iter().map(|port| {
            let (kind, identity) = match port.port_type {
                serialport::SerialPortType::UsbPort(info) => {
                    let identity = [info.manufacturer, info.product, info.serial_number]
                        .into_iter().flatten().collect::<Vec<_>>().join(" ");
                    ("usb-serial".to_string(), if identity.is_empty() { format!("USB {:04x}:{:04x}", info.vid, info.pid) } else { identity })
                }
                serialport::SerialPortType::BluetoothPort => ("bluetooth-serial".to_string(), "Bluetooth serial adapter".to_string()),
                serialport::SerialPortType::PciPort => ("pci-serial".to_string(), "PCI serial adapter".to_string()),
                serialport::SerialPortType::Unknown => ("unknown".to_string(), "Unidentified serial device".to_string()),
            };
            let searchable = format!("{} {}", kind, identity).to_ascii_lowercase();
            let (classification, safe_to_flash) = if searchable.contains("esp32") || searchable.contains("espressif") {
                ("ESP32".to_string(), true)
            } else if searchable.contains("arduino") || searchable.contains("atmega") {
                ("Arduino-compatible".to_string(), true)
            } else if searchable.contains("ch340") || searchable.contains("ftdi") || searchable.contains("cp210") {
                ("USB serial adapter — verify target".to_string(), false)
            } else {
                ("Unknown — do not flash".to_string(), false)
            };
            SerialCandidate { port: port.port_name, kind, identity, classification, safe_to_flash }
        }).collect())
    }
}

#[tauri::command]
fn scan_wifi_networks() -> Result<Vec<String>, String> {
    #[cfg(target_os = "android")]
    { return Ok(Vec::new()); }

    #[cfg(not(target_os = "android"))]
    {
        #[cfg(target_os = "windows")]
        let output = std::process::Command::new("netsh").args(["wlan", "show", "networks", "mode=bssid"]).output();
        #[cfg(all(unix, not(target_os = "android")))]
        let output = std::process::Command::new("nmcli").args(["-t", "-f", "SSID", "dev", "wifi", "list"]).output();
        #[cfg(not(any(target_os = "windows", all(unix, not(target_os = "android")))))]
        { return Err("Wi-Fi scanning is not available on this platform yet".into()); }

        let output = output.map_err(|error| format!("Wi-Fi scanner unavailable: {error}"))?;
        if !output.status.success() { return Err("Wi-Fi scan command failed; ensure Wi-Fi is enabled".into()); }
        let text = String::from_utf8_lossy(&output.stdout);
        let mut networks = Vec::new();
        for line in text.lines() {
            let candidate = if cfg!(target_os = "windows") {
                line.trim().strip_prefix("SSID ").and_then(|value| value.split_once(':').map(|(_, name)| name.trim().to_string()))
            } else { Some(line.trim().replace("\\:", ":")) };
            if let Some(name) = candidate.filter(|name| !name.is_empty() && !networks.contains(name)) { networks.push(name); }
        }
        Ok(networks)
    }
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
        .invoke_handler(tauri::generate_handler![list_devices, add_device, flash_target, list_logs, search_serial_devices, scan_wifi_networks, shell_command])
        .run(tauri::generate_context!())
        .expect("error while running RCAMP");
}
