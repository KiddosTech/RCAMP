//! Shared, local-first device and profile API used by RCAMP clients.
#![forbid(unsafe_code)]

pub mod device;
pub mod profile;

pub use device::{ConnectionState, Device, DeviceId, DeviceManager, TransportKind};
pub use profile::{DeviceProfile, ProfileError};
