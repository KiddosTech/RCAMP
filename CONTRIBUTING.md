# Contributing to RCAMP

Keep hardware access in `rcamp-core`; GUI and CLI must not add their own transport implementations. Treat all profile, network, Bluetooth and serial input as untrusted.

Before opening a pull request, run `npm run check`, `cargo fmt --check`, `cargo test -p rcamp-core -p rcamp-cli`, and `cargo clippy --workspace --all-targets` where your platform supports the required Tauri system dependencies.
