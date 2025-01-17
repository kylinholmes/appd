use std::path::PathBuf;
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use toml;
use tracing::level_filters::LevelFilter;

pub static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    CONFIG_PATH.get().map(|path| {
        std::fs::read_to_string(path).map(|config| {
            toml::from_str(&config).unwrap()
        }).unwrap()
    }).unwrap()
});

/// The configuration of the application.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// The address to bind.
    pub admin_addr: Option<String>,
    pub log: LogConfig,
    pub app: App,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct App {
    pub addr: String,
    pub config: PathBuf
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogConfig {
    pub path: PathBuf,
    pub level: LogLevel,
}

#[derive(Debug, Serialize, Deserialize, Clone,Default)]
pub enum LogLevel {
    Debug,
    #[default]
    Info,
    Warn,
    Error,
}

/// compile-time include, hard code: config path
impl Default for Config {
    fn default() -> Self {
        let c = include_str!("../config/webd.toml");
        toml::from_str(c).unwrap()
    }
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            path: "webd.log".into(),
            level: LogLevel::Info,
        }
    }
}

impl From<LogLevel> for LevelFilter {
    fn from(val: LogLevel) -> Self {
        match val {
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}