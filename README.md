# 🍅 PomoCLI

A simple, clean pomodoro timer built for developers who live in the terminal.

## ✨ Features

- 🔥 **Beautiful Terminal UI** - Colorful progress bars with real-time countdown
- 🍎 **macOS System Tray** - Live countdown in your menu bar (macOS only)
- 🎵 **Audio Notifications** - Pleasant chime when timers complete
- ⚡ **Lightning Fast** - Built with Rust for instant startup
- 🎨 **Phase Emojis** - Visual indicators for work (🔥), breaks (☕), and long breaks (😴)
- 🔄 **Full Pomodoro Cycle** - Automatic work/break transitions
- 📱 **Cross-Platform** - Works on macOS, Linux, and Windows

## 🚀 Quick Start

```bash
# Start a pomodoro session (25min work, 5min break, 4 cycles)
cargo run

# Custom durations
cargo run -- start -w 30 -s 10 -l 20

# Quick timer
cargo run -- timer 15

# Test sound
cargo run -- test-sound
```

## 🛠️ Installation

### From Source
```bash
git clone https://github.com/yourusername/pomocli
cd pomocli
cargo build --release
```

The binary will be available at `target/release/pomocli`.

## 📖 Usage

### Default Pomodoro Session
```bash
pomocli
```
Starts the classic pomodoro technique:
- 🔥 25 minutes of focused work
- ☕ 5 minute short breaks
- 😴 15 minute long break after 4 cycles

### Custom Pomodoro
```bash
# Custom work and break durations
pomocli start --work 45 --short-break 10 --long-break 30 --cycles 3

# Short form
pomocli start -w 45 -s 10 -l 30 -c 3
```

### Quick Timer
```bash
# 15-minute timer
pomocli timer 15
```

### Commands
- `start` - Begin a pomodoro session with work/break cycles
- `timer <minutes>` - Start a simple countdown timer
- `test-sound` - Test the notification sound
- `--help` - Show all available options

## 🎯 What You'll See

### Terminal Interface
```
🔥 WORK TIME (Cycle 1/4)
⠋ [00:12:34] [████████████▌               ] 754/1500 🔥 Work - 12:26 remaining
```

### macOS System Tray
Your menu bar will show live countdown:
- `🔥 24:32` during work sessions
- `☕ 4:45` during short breaks  
- `😴 14:30` during long breaks

## ⚙️ Configuration

### Default Settings
- **Work duration**: 25 minutes
- **Short break**: 5 minutes
- **Long break**: 15 minutes
- **Cycles before long break**: 4

All durations can be customized via command line arguments.

## 🔊 Audio

PomoCLI plays a pleasant notification chime when timers complete. The sound is:
- 880Hz tone (musical note A5)
- Natural exponential decay for a bell-like quality
- 3-second duration with smooth fade-out

## 🖥️ Platform Features

### macOS
- ✅ System tray integration with live countdown
- ✅ Native audio notifications
- ✅ Beautiful terminal progress bars

### Linux & Windows
- ✅ Beautiful terminal progress bars
- ✅ Audio notifications
- ⚪ System tray (not implemented)

## 🛡️ Requirements

- **Rust** 1.70+ (for building from source)
- **Audio system** (for notifications)

### macOS
No additional dependencies required.

### Linux
May require audio libraries:
```bash
# Ubuntu/Debian
sudo apt install libasound2-dev

# Fedora
sudo dnf install alsa-lib-devel
```

## 🏗️ Development

```bash
# Clone and build
git clone https://github.com/yourusername/pomocli
cd pomocli
cargo build

# Run tests
cargo test

# Format code
cargo fmt

# Lint
cargo clippy
```

## 📝 Architecture

Built with modern Rust libraries:
- **clap** - CLI argument parsing
- **tokio** - Async runtime for timers
- **indicatif** - Beautiful progress bars
- **crossterm** - Cross-platform terminal manipulation
- **rodio** - Cross-platform audio
- **tray-icon** - macOS system tray integration

## 🤝 Contributing

Contributions welcome! Please feel free to:
- Report bugs
- Request features
- Submit pull requests
- Improve documentation

## 📄 License

GPL-3.0 License - see LICENSE file for details.

## 🙏 Acknowledgments

Inspired by the Pomodoro Technique® developed by Francesco Cirillo. Built for developers who want a simple, distraction-free productivity tool.

---

**Stay focused. Stay productive. 🍅**