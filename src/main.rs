use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;
use crossterm::{
    ExecutableCommand, cursor,
    style::Stylize,
    terminal::{self, ClearType},
};
use indicatif::{ProgressBar, ProgressStyle};
use rodio::{Decoder, OutputStream, Sink};
use std::io::{Cursor, stdout};
use std::time::Duration;
use tokio::time::{Instant, sleep};
use tray_icon::TrayIconBuilder;
use winit::event_loop::{EventLoop, ControlFlow};

#[derive(Parser)]
#[command(name = "pomocli")]
#[command(about = "A beautiful pomodoro timer for developers")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Start a pomodoro session
    Start {
        /// Work duration in minutes (default: 25)
        #[arg(short, long, default_value_t = 25)]
        work: u32,
        /// Short break duration in minutes (default: 5)
        #[arg(short, long, default_value_t = 5)]
        short_break: u32,
        /// Long break duration in minutes (default: 15)
        #[arg(short, long, default_value_t = 15)]
        long_break: u32,
        /// Number of pomodoros before long break (default: 4)
        #[arg(short, long, default_value_t = 4)]
        cycles: u32,
    },
    /// Start a quick timer
    Timer {
        /// Duration in minutes
        #[arg(value_name = "MINUTES")]
        minutes: u32,
    },
    /// Test the notification sound
    TestSound,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Start {
            work,
            short_break,
            long_break,
            cycles,
        }) => {
            if cfg!(target_os = "macos") {
                start_tray_pomodoro_session(work, short_break, long_break, cycles).await?;
            } else {
                start_pomodoro_session(work, short_break, long_break, cycles).await?;
            }
        }
        Some(Commands::Timer { minutes }) => {
            if cfg!(target_os = "macos") {
                start_tray_timer(minutes, "timer").await?;
            } else {
                start_timer(minutes, "Timer").await?;
            }
        }
        Some(Commands::TestSound) => {
            println!(
                "{}",
                "🔊 Testing notification sound...".bright_cyan().bold()
            );
            play_notification_sound().await;
            println!("{}", "Sound test complete!".bright_green());
        }
        None => {
            // Default to start command with default values
            if cfg!(target_os = "macos") {
                start_tray_pomodoro_session(25, 5, 15, 4).await?;
            } else {
                start_pomodoro_session(25, 5, 15, 4).await?;
            }
        }
    }

    Ok(())
}

async fn start_pomodoro_session(
    work_minutes: u32,
    short_break_minutes: u32,
    long_break_minutes: u32,
    cycles: u32,
) -> Result<()> {
    println!("{}", "🍅 Pomodoro Session Starting!".bright_red().bold());
    println!(
        "Work: {}min | Short Break: {}min | Long Break: {}min | Cycles: {}",
        work_minutes.to_string().cyan(),
        short_break_minutes.to_string().green(),
        long_break_minutes.to_string().blue(),
        cycles.to_string().yellow()
    );
    println!();

    for cycle in 1..=cycles {
        // Work period
        println!(
            "{} {} (Cycle {}/{})",
            "🔥".bright_red(),
            "WORK TIME".bright_red().bold(),
            cycle.to_string().yellow(),
            cycles.to_string().yellow()
        );
        start_timer(work_minutes, "Work").await?;

        if cycle < cycles {
            // Short break
            println!(
                "{} {}",
                "☕".bright_green(),
                "SHORT BREAK".bright_green().bold()
            );
            start_timer(short_break_minutes, "Short Break").await?;
        }
    }

    // Long break after all cycles
    println!(
        "{} {}",
        "🎉".bright_blue(),
        "LONG BREAK - You earned it!".bright_blue().bold()
    );
    start_timer(long_break_minutes, "Long Break").await?;

    println!(
        "{}",
        "🎊 Pomodoro session complete! Great work!"
            .bright_magenta()
            .bold()
    );
    Ok(())
}

async fn start_timer(minutes: u32, label: &str) -> Result<()> {
    let total_seconds = minutes * 60;
    let total_duration = Duration::from_secs(total_seconds as u64);

    let pb = ProgressBar::new(total_seconds as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .unwrap()
            .progress_chars("██▌ "),
    );
    pb.set_message(format!("{} - {}min", label, minutes));

    let start_time = Instant::now();

    while start_time.elapsed() < total_duration {
        let elapsed = start_time.elapsed();
        let remaining = total_duration - elapsed;
        let elapsed_seconds = elapsed.as_secs();

        pb.set_position(elapsed_seconds);
        pb.set_message(format!(
            "{} - {}:{:02} remaining",
            label,
            remaining.as_secs() / 60,
            remaining.as_secs() % 60
        ));

        sleep(Duration::from_millis(100)).await;
    }

    pb.finish_with_message(format!("{} - Complete! ✅", label));

    // Play completion sound
    play_notification_sound().await;

    // Clear terminal and show completion
    stdout().execute(terminal::Clear(ClearType::All))?;
    stdout().execute(cursor::MoveTo(0, 0))?;

    println!(
        "{} {} {} completed!",
        "🎉".bright_green(),
        label.bright_cyan().bold(),
        format!("({}min)", minutes).dim()
    );
    println!();

    // Brief pause before next session
    sleep(Duration::from_secs(2)).await;

    Ok(())
}

async fn play_notification_sound() {
    // Generate a pleasant single-tone notification sound
    tokio::task::spawn_blocking(|| {
        if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
            if let Ok(sink) = Sink::try_new(&stream_handle) {
                let sample_rate = 44100;
                let frequency = 880.0; // Higher pitch (E6) - more attention-grabbing
                let duration = 3.0; // 3 seconds to ensure full decay
                let duration_samples = (sample_rate as f32 * duration) as usize;
                let mut samples = Vec::with_capacity(duration_samples);

                for i in 0..duration_samples {
                    let t = i as f32 / sample_rate as f32;
                    // Create a "ping with reverb" envelope - instant on, natural decay
                    let envelope = (-t * 3.0).exp(); // Back to the better decay rate

                    let sample =
                        (t * frequency * 2.0 * std::f32::consts::PI).sin() * 0.6 * envelope;
                    samples.push(sample);
                }

                // Convert to bytes (16-bit PCM)
                let mut pcm_data = Vec::new();
                for sample in samples {
                    let sample_i16 = (sample * 32767.0) as i16;
                    pcm_data.extend_from_slice(&sample_i16.to_le_bytes());
                }

                // Create a cursor for the PCM data with WAV header
                let mut wav_data = Vec::new();

                // WAV header
                wav_data.extend_from_slice(b"RIFF");
                wav_data.extend_from_slice(&(36 + pcm_data.len() as u32).to_le_bytes());
                wav_data.extend_from_slice(b"WAVE");
                wav_data.extend_from_slice(b"fmt ");
                wav_data.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
                wav_data.extend_from_slice(&1u16.to_le_bytes()); // audio format (PCM)
                wav_data.extend_from_slice(&1u16.to_le_bytes()); // channels
                wav_data.extend_from_slice(&(sample_rate as u32).to_le_bytes());
                wav_data.extend_from_slice(&(sample_rate as u32 * 2).to_le_bytes()); // byte rate
                wav_data.extend_from_slice(&2u16.to_le_bytes()); // block align
                wav_data.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
                wav_data.extend_from_slice(b"data");
                wav_data.extend_from_slice(&(pcm_data.len() as u32).to_le_bytes());
                wav_data.extend_from_slice(&pcm_data);

                let cursor = Cursor::new(wav_data);
                if let Ok(source) = Decoder::new(cursor) {
                    sink.append(source);
                    sink.sleep_until_end();
                }
            }
        }
    })
    .await
    .ok();
}

async fn start_tray_timer(minutes: u32, timer_type: &str) -> Result<()> {
    let (emoji, phase_name) = match timer_type {
        "work" => ("🔥", "Work"),
        "break" => ("☕", "Break"),
        "longbreak" => ("😴", "Long Break"),
        _ => ("⏰", "Timer"),
    };
    
    
    // Create event loop for tray
    let event_loop = EventLoop::new().unwrap();
    
    // Create tray icon
    let tray_icon = TrayIconBuilder::new()
        .with_title(format!("{} {}:00", emoji, minutes))
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to create tray icon: {}", e))?;
    
    let total_seconds = minutes * 60;
    let start_time = Instant::now();
    
    // Create progress bar
    let pb = ProgressBar::new(total_seconds as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}")
            .unwrap()
            .progress_chars("██▌ "),
    );
    pb.set_message(format!("{} {} - {}min", emoji, phase_name, minutes));
    
    
    // Run event loop with timer updates
    let mut last_progress_update = Instant::now();
    let mut last_tray_update = Instant::now();
    
    event_loop.run(move |_event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);
        
        let now = Instant::now();
        let elapsed = start_time.elapsed();
        let total_duration = Duration::from_secs(total_seconds as u64);
        
        if elapsed >= total_duration {
            // Timer complete
            let _ = tray_icon.set_title(Some("✅ Complete!"));
            pb.finish_with_message(format!("{} {} - Complete! ✅", emoji, phase_name));
            
            // Play sound in a blocking thread
            std::thread::spawn(|| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                rt.block_on(async {
                    play_notification_sound().await;
                });
            });
            
            // Brief pause then exit
            std::thread::spawn(|| {
                std::thread::sleep(Duration::from_secs(2));
                std::process::exit(0);
            });
            return;
        }
        
        let remaining = total_duration - elapsed;
        let minutes_left = remaining.as_secs() / 60;
        let seconds_left = remaining.as_secs() % 60;
        let elapsed_seconds = elapsed.as_secs();
        
        // Update progress bar every 100ms
        if now.duration_since(last_progress_update) >= Duration::from_millis(100) {
            pb.set_position(elapsed_seconds);
            pb.set_message(format!(
                "{} {} - {}:{:02} remaining",
                emoji,
                phase_name,
                minutes_left,
                seconds_left
            ));
            last_progress_update = now;
        }
        
        // Update tray every second
        if now.duration_since(last_tray_update) >= Duration::from_secs(1) {
            let title = if remaining.as_secs() < 60 {
                format!("{} 0:{:02}", emoji, seconds_left)
            } else {
                format!("{} {}:{:02}", emoji, minutes_left, seconds_left)
            };
            let _ = tray_icon.set_title(Some(&title));
            last_tray_update = now;
        }
    }).map_err(|e| anyhow::anyhow!("Event loop error: {}", e))?;
    
    Ok(())
}

async fn start_tray_pomodoro_session(
    work_minutes: u32,
    short_break_minutes: u32,
    long_break_minutes: u32,
    cycles: u32,
) -> Result<()> {
    println!("{}", "🍅 Pomodoro Session Starting!".bright_red().bold());
    println!(
        "Work: {}min | Short Break: {}min | Long Break: {}min | Cycles: {}",
        work_minutes.to_string().cyan(),
        short_break_minutes.to_string().green(),
        long_break_minutes.to_string().blue(),
        cycles.to_string().yellow()
    );
    println!();

    for cycle in 1..=cycles {
        // Work period
        println!(
            "{} {} (Cycle {}/{})",
            "🔥".bright_red(),
            "WORK TIME".bright_red().bold(),
            cycle.to_string().yellow(),
            cycles.to_string().yellow()
        );
        start_tray_timer(work_minutes, "work").await?;

        if cycle < cycles {
            // Short break
            println!(
                "{} {}",
                "☕".bright_green(),
                "SHORT BREAK".bright_green().bold()
            );
            start_tray_timer(short_break_minutes, "break").await?;
        }
    }

    // Long break after all cycles
    println!(
        "{} {}",
        "😴".bright_blue(),
        "LONG BREAK - You earned it!".bright_blue().bold()
    );
    start_tray_timer(long_break_minutes, "longbreak").await?;

    println!("{}", "🎊 Pomodoro session complete! Great work!".bright_magenta().bold());
    Ok(())
}
