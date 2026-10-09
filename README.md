<p align="center"><img src="assets/rcamp-mark.svg" width="128" alt="RCAMP logo"></p>
<h1 align="center">RCAMP</h1>
<p align="center"><strong>One controller. Many devices. Local by default.</strong></p>
<p align="center">Open-source hardware control for ESP32, Arduino, robots, RC cars, serial devices, and custom targets.</p>
<p align="center"><a href="https://github.com/KiddosTech/RCAMP/releases"><img src="https://img.shields.io/badge/release-0.1.0-1768ee?style=flat-square" alt="RCAMP 0.1.0"></a> <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-8dffcf?style=flat-square" alt="Apache-2.0 license"></a> <img src="https://img.shields.io/badge/platform-Linux%20%7C%20Windows%20%7C%20Android-101827?style=flat-square" alt="Platforms"> <img src="https://img.shields.io/badge/status-foundation-101827?style=flat-square" alt="Foundation status"></p>

RCAMP is a local-first control plane with a shared Rust core. The desktop GUI, Android app, and RCAMP/CLI speak through the same device model instead of duplicating hardware logic.

```text
              ┌── RCAMP GUI ───── Linux · Windows · Android
              │
rcamp-core ───┼── RCAMP/CLI ───── Linux · Windows
              │
              └── RCAMP/RTOS ──── ESP32 · Arduino · targets
```

No RCAMP cloud account is required for basic operation. Your device can communicate with your computer over Wi-Fi, Bluetooth, USB, or serial.

## Product surface

| Surface | Purpose |
| --- | --- |
| **RCAMP GUI** | Tauri 2 + Svelte workspace with HQ, Tools, Settings, Preferences, Plugin, and About tabs |
| **RCAMP/CLI** | Rust command interface and keyboard-first TUI for Linux and Windows |
| **rcamp-core** | Shared device manager, profile validation, transport model, and flashing API |
| **RCAMP/RTOS** | Reference ESP32 Wi-Fi/TCP and Arduino USB/Serial target firmware |

The GUI also includes a structured **Logs** tab and an **RCAMP/RTOS Shell** tab. The shell is a safe simulated target console: it sends an allowlisted command set (`help`, `status`, `info`, `capabilities`, `ping`, `logs`, `whoami`, `rcampfetch`, `task`, and guarded `sudo`) over a direct, line-flushed TCP connection. It never executes a command on the host computer.

## RCAMP/RTOS flashing

The GUI Tools tab and CLI can flash target firmware using native vendor tools. RCAMP never evaluates a device-supplied shell command; it invokes a fixed tool with validated arguments. The Shell and Logs tabs provide realtime target interaction and structured events. `rcamp rcampfetch <address>` reports host and target details.

```sh
rcamp target flash --target esp32 --port COM3 --firmware rcamp-rtos.bin
rcamp target flash --target arduino --port COM4 --firmware rcamp-rtos.hex
```

Install `esptool` for ESP32 or `avrdude` for Arduino, then see [`rcamp-rtos/README.md`](rcamp-rtos/README.md) for firmware sketches and the target line protocol.

## Install RCAMP/CLI

The release installers download signed-by-release artifacts and do not require a local Rust installation.

Windows PowerShell:

```powershell
irm https://raw.githubusercontent.com/KiddosTech/RCAMP/main/scripts/install.ps1 | iex
```

Linux or macOS:

```sh
curl -fsSL https://raw.githubusercontent.com/KiddosTech/RCAMP/main/scripts/install.sh | sh
# or: wget -qO- https://raw.githubusercontent.com/KiddosTech/RCAMP/main/scripts/install.sh | sh
```

## Device profiles

Profiles describe capability without assuming that all hardware shares one protocol:

```json
{
  "name": "Workshop RC Car",
  "type": "rc-car",
  "connection": "tcp",
  "address": "192.168.1.42:8080",
  "capabilities": ["drive", "light", "telemetry"],
  "controls": { "motor_left": "motor_left", "motor_right": "motor_right" }
}
```

Modelled transports include TCP, UDP, HTTP, WebSocket, MQTT, serial, USB-to-serial, and BLE GATT. Adapters are capability-driven and treat network, Bluetooth, serial, and profile input as untrusted.

## Development

Install Node.js and a current Rust toolchain:

```sh
npm install
npm run check
npm run build
cargo fmt --check
cargo test -p rcamp-core -p rcamp-cli
cargo clippy --workspace --all-targets -- -D warnings
```

Start the GUI or CLI locally:

```sh
npm run tauri dev
cargo run -p rcamp-cli --
```

## Repository map

```text
crates/rcamp-core/       Shared Rust device, profile, transport, and flashing logic
crates/rcamp-cli/        RCAMP/CLI parser and TUI
src/                     Svelte + TypeScript GUI
src-tauri/               Tauri desktop/mobile host and commands
rcamp-rtos/              ESP32 and Arduino reference firmware
website/                 Multi-page project website
scripts/                 CLI installers
```

## Website

The project website is a framework-free static site with Product, About, RCAMP/CLI, and Docs pages. Deploy `website/` with any static host such as Cloudflare Pages.

## Contributing

Keep hardware access inside `rcamp-core`; avoid implementing a second transport stack in the GUI or CLI. Read [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md) before opening a change.

## License

RCAMP is licensed under the [Apache License 2.0](LICENSE).
