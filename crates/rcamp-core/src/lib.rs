//! Shared, local-first device and profile API used by RCAMP clients.
#![forbid(unsafe_code)]

pub mod device;
pub mod flashing;
pub mod profile;
pub mod shell;

pub use device::{ConnectionState, Device, DeviceId, DeviceManager, TransportKind};
pub use flashing::{flash_firmware, FlashError, FlashRequest, FlashTarget};
pub use profile::{DeviceProfile, ProfileError};
pub use shell::{execute_shell, ShellError, ShellResponse};
