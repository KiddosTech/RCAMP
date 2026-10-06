use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{ConnectionState, Device, TransportKind};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub name: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub connection: TransportKind,
    pub address: String,
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub controls: serde_json::Map<String, serde_json::Value>,
    #[serde(default)]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("profile name cannot be empty")]
    EmptyName,
    #[error("device address cannot be empty")]
    EmptyAddress,
}

impl DeviceProfile {
    pub fn validate(&self) -> Result<(), ProfileError> {
        if self.name.trim().is_empty() { return Err(ProfileError::EmptyName); }
        if self.address.trim().is_empty() { return Err(ProfileError::EmptyAddress); }
        Ok(())
    }

    #[must_use]
    pub fn to_device(&self) -> Device {
        Device { id: Uuid::new_v4(), name: self.name.clone(), device_type: self.device_type.clone(), transport: self.connection, address: self.address.clone(), state: ConnectionState::Disconnected, capabilities: self.capabilities.clone() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_empty_addresses() {
        let profile = DeviceProfile {
            name: "Car".into(), device_type: "rc-car".into(), connection: TransportKind::Tcp,
            address: " ".into(), capabilities: vec![], commands: vec![],
            controls: serde_json::Map::new(), metadata: serde_json::Map::new(),
        };
        assert!(matches!(profile.validate(), Err(ProfileError::EmptyAddress)));
    }
}
