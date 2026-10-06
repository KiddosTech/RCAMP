use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type DeviceId = Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TransportKind {
    Tcp,
    Udp,
    Http,
    WebSocket,
    Mqtt,
    Serial,
    BleGatt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: DeviceId,
    pub name: String,
    pub device_type: String,
    pub transport: TransportKind,
    pub address: String,
    pub state: ConnectionState,
    #[serde(default)]
    pub capabilities: Vec<String>,
}

/// In-memory manager. Persistent configuration and transport adapters are added behind
/// this API so GUI and CLI retain one hardware-facing interface.
#[derive(Debug, Default)]
pub struct DeviceManager {
    devices: BTreeMap<DeviceId, Device>,
}

impl DeviceManager {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, device: Device) { self.devices.insert(device.id, device); }

    #[must_use]
    pub fn devices(&self) -> Vec<&Device> { self.devices.values().collect() }

    pub fn set_state(&mut self, id: DeviceId, state: ConnectionState) -> Option<()> {
        self.devices.get_mut(&id).map(|device| device.state = state)
    }
}
