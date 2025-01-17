use std::path::PathBuf;

use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};


pub static CONFIG_PATH: OnceCell<PathBuf> = OnceCell::new();
pub static CONFIG: Lazy<AppdConfig> = Lazy::new(|| {
    CONFIG_PATH.get().map(|path| {
        std::fs::read_to_string(path).map(|config| {
            toml::from_str(&config).unwrap()
        }).unwrap()
    }).unwrap()
});

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppdConfig {
    pub db: DB
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DB {
    pub url: String
}



impl Default for AppdConfig {
    fn default() -> Self {
        let tm = include_str!("../config/appd.toml");
        toml::from_str(tm).unwrap()
    }
}