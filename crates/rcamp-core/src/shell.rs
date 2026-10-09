//! Safe, target-facing simulated shell. This never executes commands on the host OS.

use std::{io::{Read, Write}, net::{SocketAddr, TcpStream}, time::Duration};
use thiserror::Error;

const TASKS: &[&str] = &["rgb_on", "rgb_off", "rgb_set", "led_on", "led_off", "buzzer_beep", "relay_on", "relay_off", "test_write", "read_input", "sensor_read", "status_snapshot", "telemetry_on", "telemetry_off", "identify", "uptime", "storage_info", "reboot"];
const ALLOWED_COMMANDS: &[&str] = &["help", "status", "info", "capabilities", "ping", "logs", "whoami", "rcampfetch", "task", "sudo"];

#[derive(Debug, Clone)]
pub struct ShellResponse { pub output: String, pub elapsed_ms: u128 }

#[derive(Debug, Error)]
pub enum ShellError {
    #[error("target address is invalid: {0}")]
    InvalidAddress(String),
    #[error("command is not allowed by the RCAMP/RTOS shell")]
    CommandNotAllowed,
    #[error("target connection failed: {0}")]
    Connection(#[from] std::io::Error),
}

/// Send one allowlisted command to an RCAMP/RTOS target over TCP.
/// The host does not interpret the command as a shell command and the target
/// is asked to flush its response immediately.
pub fn execute_shell(address: &str, command: &str) -> Result<ShellResponse, ShellError> {
    let first = command.split_whitespace().next().unwrap_or_default().to_ascii_lowercase();
    if !ALLOWED_COMMANDS.contains(&first.as_str()) { return Err(ShellError::CommandNotAllowed); }
    let argument = command.split_whitespace().nth(1).unwrap_or_default().to_ascii_lowercase();
    if first == "task" && !TASKS.contains(&argument.as_str()) { return Err(ShellError::CommandNotAllowed); }
    if first == "sudo" && argument.is_empty() { return Err(ShellError::CommandNotAllowed); }
    let socket: SocketAddr = address.parse().map_err(|_| ShellError::InvalidAddress(address.into()))?;
    let started = std::time::Instant::now();
    let mut stream = TcpStream::connect_timeout(&socket, Duration::from_secs(3))?;
    stream.set_read_timeout(Some(Duration::from_secs(3)))?;
    stream.set_write_timeout(Some(Duration::from_secs(3)))?;
    stream.set_nodelay(true)?;
    let outbound = if first == "task" || first == "sudo" { command.trim().to_ascii_uppercase() } else { first.to_ascii_uppercase() };
    stream.write_all(format!("{outbound}\n").as_bytes())?;
    stream.flush()?;
    let mut bytes = [0_u8; 4096];
    let count = stream.read(&mut bytes).unwrap_or(0);
    Ok(ShellResponse { output: String::from_utf8_lossy(&bytes[..count]).into_owned(), elapsed_ms: started.elapsed().as_millis() })
}
