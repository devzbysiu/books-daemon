use anyhow::{bail, Result};
use config::{Config, Environment, File};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub(crate) struct Settings {
    interval: i64,
    books_dir: String,
    device_mac: String,
    stdout_file: String,
    stderr_file: String,
}

impl Settings {
    pub(crate) fn load() -> Result<Self> {
        Ok(Config::builder()
            .add_source(File::with_name(&config_path()?))
            .add_source(Environment::with_prefix("BOOKS_DAEMON"))
            .build()?
            .into())
    }

    fn new(config: &Config) -> Result<Self> {
        Ok(Self {
            interval: config.get_int("interval")?,
            books_dir: config.get_string("books_dir")?,
            device_mac: config.get_string("device_mac")?,
            stdout_file: config.get_string("stdout_file")?,
            stderr_file: config.get_string("stderr_file")?,
        })
    }

    #[allow(clippy::cast_sign_loss)]
    pub(crate) fn interval(&self) -> u64 {
        self.interval as u64
    }

    pub(crate) fn books_dir(&self) -> &str {
        &self.books_dir
    }

    pub(crate) fn device_mac(&self) -> &str {
        &self.device_mac
    }

    pub(crate) fn stdout_file(&self) -> &str {
        &self.stdout_file
    }

    pub(crate) fn stderr_file(&self) -> &str {
        &self.stderr_file
    }
}

impl From<Config> for Settings {
    fn from(cfg: Config) -> Self {
        Self::new(&cfg).expect("failed to create settings from config")
    }
}

pub(crate) fn config_path() -> Result<String> {
    if let Some(config_dir) = dirs::config_dir() {
        return Ok(format!("{}/books-daemon.toml", into_string(config_dir)));
    }
    bail!("failed to read config directory")
}

fn into_string(path: PathBuf) -> String {
    path.into_os_string()
        .into_string()
        .expect("failed to convert os string to string")
}
