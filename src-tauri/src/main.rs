#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rcamp_core::{ConnectionState, Device, DeviceManager, DeviceProfile, TransportKind};
use serde::Serialize;
use std::sync::Mutex;
use tauri::State;

struct AppState(Mutex<DeviceManager>);

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
    let manager = state.0.lock().map_err(|_| "device manager unavailable")?;
    Ok(manager.devices().into_iter().map(|device: &Device| DeviceView {
        name: device.name.clone(), device_type: device.device_type.clone(), transport: device.transport,
        address: device.address.clone(), state: device.state,
    }).collect())
}

#[tauri::command]
fn add_device(profile: DeviceProfile, state: State<'_, AppState>) -> Result<DeviceView, String> {
    profile.validate().map_err(|error| error.to_string())?;
    let device = profile.to_device();
    let view = DeviceView { name: device.name.clone(), device_type: device.device_type.clone(), transport: device.transport, address: device.address.clone(), state: device.state };
    let mut manager = state.0.lock().map_err(|_| "device manager unavailable")?;
    manager.register(device);
    Ok(view)
}

fn main() {
    tracing_subscriber::fmt::init();
    tauri::Builder::default()
        .manage(AppState(Mutex::new(DeviceManager::new())))
        .invoke_handler(tauri::generate_handler![list_devices, add_device])
        .run(tauri::generate_context!())
        .expect("error while running RCAMP");
}
