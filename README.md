# HotCorner Win

HotCorner Win is inspired by the Hot Corners feature commonly found on Linux desktop environments.

# Features

- Configure all four screen corners
- Show the desktop with "Win + D"
- Open Task View with "Win + Tab"
- Lock Windows
- Launch an application
- Customize the activation delay
- Customize the detection zone size
- Multi-monitor support
- Automatic configuration saving
- Runs in the background using the System Tray

# Building

The project requires Rust and Cargo.

To run the application:

cargo run

To build the release executable:

cargo build --release

The executable will be generated at:

target/release/hotcorner-win.exe

# Project Structure

src/
├── ui/
│   ├── mod.rs
│   └── settings.rs
├── action.rs
├── config.rs
├── hotcorner.rs
├── main.rs
└── tray.rs

# Configuration

Settings are automatically saved to:

config.json

They can be modified directly through the application's interface.

# Technologies

- Rust
- Win32 API
- egui / eframe
- tray-icon
- Serde