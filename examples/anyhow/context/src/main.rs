use anyhow::{Context, Result};
use std::fs;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
enum ConfigError {
    #[error("Configuration file not found")]
    FileNotFound,
    #[error("Permission denied accessing config")]
    PermissionDenied,
    #[error("Config file is corrupted")]
    Corrupted,
}

fn read_config() -> Result<String> {
    fs::read_to_string("config.txt")
        .map_err(|e| match e.kind() {
            std::io::ErrorKind::NotFound => ConfigError::FileNotFound,
            std::io::ErrorKind::PermissionDenied => ConfigError::PermissionDenied,
            _ => ConfigError::Corrupted,
        })
        .context("Failed to load application configuration")
}

fn main() {
    match read_config() {
        Ok(content) => println!("Config: {}", content),
        Err(e) => {
            println!("Error occurred: {}", e);
            
            // Check for specific error types using match
            if let Some(config_error) = e.downcast_ref::<ConfigError>() {
                match config_error {
                    ConfigError::FileNotFound => {
                        println!("🔍 Matched FileNotFound error!");
                        println!("Action: Creating default config file...");
                        // Could create default config here
                    },
                    ConfigError::PermissionDenied => {
                        println!("🔒 Matched PermissionDenied error!");
                        println!("Action: Check file permissions");
                    },
                    ConfigError::Corrupted => {
                        println!("💥 Matched Corrupted error!");
                        println!("Action: Backup and recreate config");
                    }
                }
            } else {
                println!("❓ Unknown error type");
            }
            
            // Also check the error chain
            println!("\nError chain:");
            let mut source = e.source();
            let mut level = 1;
            while let Some(err) = source {
                println!("  Level {}: {}", level, err);
                source = err.source();
                level += 1;
            }
        }
    }
}
