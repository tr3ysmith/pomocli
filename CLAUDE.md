# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A beautiful cross-platform pomodoro timer CLI for developers. Built with Rust, featuring colorful terminal output, progress bars, and async timers.

## Essential Commands

### Build and Development
- `cargo build` - Compile the project
- `cargo run` - Build and run the application
- `cargo check` - Check code for errors without building
- `cargo clean` - Remove build artifacts

### Running the Application
- `cargo run -- start` - Start a full pomodoro session (25min work, 5min break, 4 cycles)
- `cargo run -- start -w 30 -s 10` - Custom work/break times
- `cargo run -- timer 15` - Quick 15-minute timer
- `cargo run -- --help` - Show help

### Testing and Quality
- `cargo test` - Run tests
- `cargo clippy` - Run Rust linter
- `cargo fmt` - Format code

## Key Dependencies

- `clap` - CLI argument parsing with derive macros
- `tokio` - Async runtime for timers
- `indicatif` - Progress bars
- `colored` - Terminal color support
- `crossterm` - Cross-platform terminal manipulation
- `anyhow` - Error handling

## Code Structure

- `src/main.rs` - Main application with CLI definition and timer logic
- Two main commands:
  - `start` - Full pomodoro session with work/break cycles
  - `timer` - Simple countdown timer

## Architecture Notes

- Async-based timer implementation using `tokio::time`
- Progress bars with real-time updates (100ms intervals)
- Terminal clearing and cursor manipulation for clean UI
- Colorful output with emojis for better UX