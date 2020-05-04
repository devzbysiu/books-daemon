use anyhow::{bail, Result};
use config::{Config, File};
use dirs;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    interval: u64,
    books_dir: String,
    device_mac: String,
}

impl Settings {
    pub(crate) fn load() -> Result<Self> {
        let mut config = Config::new();
        config.merge(File::with_name(&config_path()?))?;
        match config.try_into() {
            Ok(config) => Ok(config),
            Err(e) => bail!("failed to read config: {}", e),
        }
    }

    pub(crate) fn interval(&self) -> u64 {
        self.interval
    }

    pub(crate) fn books_dir(&self) -> &str {
        &self.books_dir
    }

    pub(crate) fn device_mac(&self) -> &str {
        &self.device_mac
    }
}

fn config_path() -> Result<String> {
    if let Some(config_dir) = dirs::config_dir() {
        return Ok(format!("{}/books-daemon.toml", into_string(config_dir)?));
    }
    bail!("Failed to read config directory")
}

fn into_string(path: PathBuf) -> Result<String> {
    Ok(path
        .into_os_string()
        .into_string()
        .expect("failed to convert os string to string"))
}
