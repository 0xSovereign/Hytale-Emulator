//! SOVEREIGN Emulator Launcher
//!
//! A simple and fast emulator for Hytale

mod modules;
use crate::modules::config::SovereignConfig;
use std::process::Command;
use std::fs::OpenOptions;
use std::io::Write;

/// A logger that's ran in debug mode.
struct Logger;
impl Logger {
    fn log(msg: &str) {
        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("sovereign.log") {
            let _ = writeln!(file, "[{}] {}", chrono::Local::now().format("%H:%M:%S"), msg);
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Load user config
    let config = match SovereignConfig::load("SOVEREIGN.ini") {
        Ok(c) => {
            Logger::log("Ini loaded.");
            c
        },
        Err(e) => {
            Logger::log(&format!("Failed to load INI: {}", e));
            return Err(e);
        }
    };

    // Prepare Hytale process
    let game_exe = "./HytaleClient.exe";
    let mut cmd = Command::new(game_exe);

    let mut arg_list = String::new();
    for (flag, value) in config.args {
        cmd.arg(&flag);
        arg_list.push_str(&format!("{} ", flag));
        
        if let Some(val) = value {
            // Only attach the value if it's not a simple boolean 'true' toggle
            if val.to_lowercase() != "true" {
                cmd.arg(&val);
                arg_list.push_str(&format!("{} ", val));
            }
        }
    }
    Logger::log(&format!("Full Arguments: {}", arg_list));

    // Start the game
    Logger::log("Spawning HytaleClient.exe...");
    match cmd.spawn() {
        Ok(child) => {
            Logger::log(&format!("PID: {}", child.id()));
        }
        Err(e) => {
            Logger::log(&format!("Game failed to start. Reason: {}", e));
        }
    }

    Ok(())
}