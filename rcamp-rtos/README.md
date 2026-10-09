# RCAMP/RTOS Target Firmware

RCAMP/RTOS is a lightweight embedded runtime layer for devices controlled by RCAMP. It is intentionally small and transparent: the target exposes a realtime TCP/Serial line interface, reports capabilities, emits structured logs, and treats every received command as untrusted input. The ESP32 reference uses a dedicated FreeRTOS network task; the Arduino reference uses a cooperative serial task loop.

This directory contains starter sketches for:

- ESP32 over Wi-Fi/TCP
- Arduino Nano over USB/Serial

The firmware examples are a starting point for target developers, not a replacement for the board vendor's bootloader. Build a `.bin` for ESP32 or `.hex` for Arduino, then flash it with RCAMP/CLI:

```sh
rcamp target flash --target esp32 --port COM3 --firmware rcamp-rtos.bin
rcamp target flash --target arduino --port COM4 --firmware rcamp-rtos.hex
```

The desktop GUI exposes the same flashing operation through its Tools tab. The host must have `esptool` (ESP32) or `avrdude` (Arduino) on `PATH`.

## Target line protocol

Commands are newline-delimited UTF-8 text:

```text
PING          -> PONG
INFO          -> {"name":"...","firmware":"RCAMP/RTOS","capabilities":[...]}
CAPABILITIES  -> JSON capability list
STATUS        -> JSON health and uptime
LOGS          -> newline-delimited target events
WHOAMI        -> current role
RCAMPFETCH    -> target and host-facing system summary
TASK RGB_ON   -> example capability task
TASK TEST_WRITE -> protected write test
SUDO <token>  -> elevate User to Administrator after provisioning
```

Device-specific commands should be capability-driven and validated by the target. Do not execute received text as a shell command.

## Task catalog

The reference runtime exposes these task names. A target may report a task as `simulated` until its board-specific driver is configured:

| Task | Purpose |
| --- | --- |
| `rgb_on` / `rgb_off` | Turn the configured RGB/LED output on or off |
| `rgb_set` | Set an RGB value through the board driver |
| `led_on` / `led_off` | Control a simple status LED |
| `buzzer_beep` | Trigger a buzzer pulse |
| `relay_on` / `relay_off` | Switch a relay output |
| `test_write` | Verify a protected write path (Administrator required) |
| `read_input` | Read a configured digital input |
| `sensor_read` | Read the configured sensor provider |
| `status_snapshot` | Return health, heap, and readiness data |
| `telemetry_on` / `telemetry_off` | Enable or disable telemetry streaming |
| `identify` | Visually identify the target |
| `uptime` | Return target runtime duration |
| `storage_info` | Report target storage mode and capacity |
| `reboot` | Schedule a controlled reboot (Administrator required) |

The RCAMP GUI **Shell** tab is a simulated target shell, not a host operating-system terminal. It only sends the allowlisted commands above, uses a direct TCP connection with `TCP_NODELAY`, and displays each response as soon as it arrives.

## Roles

Every target has a `root`, `administrator`, or `user` role. `root` is reserved for the device runtime, normal sessions start as `user`, and protected tasks require `administrator`. Configure an administrator token during provisioning; the sketches intentionally ship with `CHANGE_ME`, which cannot elevate a session.
