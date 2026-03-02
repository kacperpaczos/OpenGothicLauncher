use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use dirs::config_local_dir;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] toml::de::Error),
    #[error("Failed to serialize config: {0}")]
    SerializeError(#[from] toml::ser::Error),
    #[error("Configuration directory not found")]
    NoConfigDir,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LauncherConfig {
    pub gothic_path: Option<PathBuf>,
    pub active_engine: Option<String>,
    pub active_profile: Option<String>,
}

pub struct ConfigManager {
    cfg_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let base_dir = config_local_dir().ok_or(ConfigError::NoConfigDir)?;
        let cfg_dir = base_dir.join("OpenGothicLauncher");
        if !cfg_dir.exists() {
            std::fs::create_dir_all(&cfg_dir)?;
        }
        Ok(Self { cfg_dir })
    }

    pub fn config_path(&self) -> PathBuf {
        self.cfg_dir.join("config.toml")
    }

    pub fn load(&self) -> Result<LauncherConfig, ConfigError> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(LauncherConfig::default());
        }
        let content = std::fs::read_to_string(path)?;
        let cfg = toml::from_str(&content)?;
        Ok(cfg)
    }

    pub fn save(&self, config: &LauncherConfig) -> Result<(), ConfigError> {
        let path = self.config_path();
        let content = toml::to_string(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
