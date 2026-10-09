//! Safe wrappers around the platform flashing tools used by RCAMP/RTOS targets.
//! No shell is invoked: executable names and arguments are constructed explicitly.

use std::{path::PathBuf, process::Command};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlashTarget { Esp32, Arduino }

#[derive(Debug, Clone)]
pub struct FlashRequest {
    pub target: FlashTarget,
    pub port: String,
    pub firmware: PathBuf,
}

#[derive(Debug, Error)]
pub enum FlashError {
    #[error("firmware file does not exist: {0}")]
    MissingFirmware(PathBuf),
    #[error("firmware extension is not valid for this target")]
    InvalidFirmware,
    #[error("serial port cannot be empty")]
    EmptyPort,
    #[error("flashing tool is not installed or not available on PATH: {0}")]
    ToolUnavailable(String),
    #[error("flashing failed with status {status}: {output}")]
    Failed { status: i32, output: String },
    #[error("could not start flashing tool: {0}")]
    Start(#[from] std::io::Error),
}

impl FlashRequest {
    fn validate(&self) -> Result<(), FlashError> {
        if self.port.trim().is_empty() { return Err(FlashError::EmptyPort); }
        if !self.firmware.is_file() { return Err(FlashError::MissingFirmware(self.firmware.clone())); }
        let extension = self.firmware.extension().and_then(|value| value.to_str()).unwrap_or_default().to_ascii_lowercase();
        let valid = match self.target { FlashTarget::Esp32 => extension == "bin", FlashTarget::Arduino => extension == "hex" };
        if !valid { return Err(FlashError::InvalidFirmware); }
        Ok(())
    }
}

/// Flash an RCAMP/RTOS image using an installed native flashing utility.
pub fn flash_firmware(request: &FlashRequest) -> Result<String, FlashError> {
    request.validate()?;
    let firmware = request.firmware.to_string_lossy().into_owned();
    let (tool, args): (&str, Vec<String>) = match request.target {
        FlashTarget::Esp32 => ("esptool", vec!["--chip".into(), "esp32".into(), "--port".into(), request.port.clone(), "write-flash".into(), "0x0".into(), firmware]),
        FlashTarget::Arduino => ("avrdude", vec!["-p".into(), "atmega328p".into(), "-c".into(), "arduino".into(), "-P".into(), request.port.clone(), "-U".into(), format!("flash:w:{firmware}:i")]),
    };
    let output = Command::new(tool).args(&args).output().map_err(|error| {
        if error.kind() == std::io::ErrorKind::NotFound { FlashError::ToolUnavailable(tool.into()) } else { FlashError::Start(error) }
    })?;
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    if !output.status.success() { return Err(FlashError::Failed { status: output.status.code().unwrap_or(-1), output: text }); }
    Ok(text)
}
