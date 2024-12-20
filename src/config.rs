use serde::Deserialize;
use crate::error::{ElizaCliError, Result};
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log_level: String,
    pub data_path: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
            data_path: "./data".to_string(),
        }
    }
}

impl Config {
    pub fn load_from_file(path: &str) -> Result<Self> {
        let data = fs::read_to_string(path)
            .map_err(|e| ElizaCliError::ConfigError(format!("Failed to read config: {}", e)))?;
        let config: Config = serde_json::from_str(&data)
            .map_err(|e| ElizaCliError::ConfigError(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }
}
