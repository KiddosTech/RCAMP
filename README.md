# RCAMP

RCAMP is a FOSS, local-first platform for managing hardware such as ESP32 devices, Arduino projects, robots, RC cars and serial/network equipment. It does not require a cloud account for normal device control.

## Components

- **RCAMP GUI** — Tauri 2 + Svelte application for Linux, Windows and Android.
- **RCAMP/CLI** — Rust command line application with an interactive TUI for Linux and Windows.
- **rcamp-core** — shared Rust device/profile model used by both interfaces.

## Supported transport model

The core models TCP, UDP, HTTP, WebSocket, MQTT, serial and BLE GATT as distinct transports. Transport adapters and discovery are deliberately capability-driven: a profile never implies that another device uses the same protocol.

## Development

Install a current Rust toolchain and Node.js, then:

```sh
npm install
npm run check
cargo fmt --check
cargo test -p rcamp-core -p rcamp-cli
npm run tauri dev
```

Run `cargo run -p rcamp-cli --` to open **RCAMP/CLI**, or append `devices list`, `devices discover`, `devices info <device>`, `doctor`, or `version`. The initial GUI profile form keeps devices for the current application session; durable profile storage is the next core milestone.

## Device profiles

Profiles are validated data rather than executable device code. A minimal example:

```json
{
  "name": "My RC Car",
  "type": "rc-car",
  "connection": "tcp",
  "address": "192.168.1.42:80",
  "capabilities": ["drive"],
  "controls": { "motor_left": "motor_left", "motor_right": "motor_right" }
}
```

## Architecture

```text
Svelte GUI / Android / RCAMP-CLI
              │
          rcamp-core
              │
  Device manager → transport adapter → hardware
```

## Builds

GitHub Actions produces a Linux AppImage, Windows MSI, Android APK and AAB, plus standalone Linux and Windows CLI artifacts. Android release signing material is supplied only through GitHub Actions secrets; it is never committed.

See [CONTRIBUTING.md](CONTRIBUTING.md) and [SECURITY.md](SECURITY.md). Licensed under [Apache-2.0](LICENSE).
