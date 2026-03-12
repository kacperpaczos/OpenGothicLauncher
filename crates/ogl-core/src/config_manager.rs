use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use dirs::config_local_dir;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Configuration directory not found")]
    NoConfigDir,
}

/// Per-game persistent state (serialized in state.json).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameState {
    /// Detected installation path on disk.
    pub install_path: Option<PathBuf>,
    /// Whether the game was successfully detected.
    pub detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LauncherConfig {
    /// Currently selected OpenGothic engine version tag (e.g. "v1.0.4").
    pub active_engine: Option<String>,
    /// Currently active sandbox/profile name.
    pub active_profile: Option<String>,
    /// Per-game detection state, keyed by game variant name.
    pub games: HashMap<String, GameState>,
}

pub struct ConfigManager {
    cfg_dir: PathBuf,
}

impl ConfigManager {
    pub fn new() -> Result<Self, ConfigError> {
        let base_dir = config_local_dir().ok_or(ConfigError::NoConfigDir)?;
        let cfg_dir = base_dir.join("OpenGothicLauncher");
        Self::with_dir(cfg_dir)
    }

    pub fn with_dir(cfg_dir: PathBuf) -> Result<Self, ConfigError> {
        if !cfg_dir.exists() {
            std::fs::create_dir_all(&cfg_dir)?;
        }
        Ok(Self { cfg_dir })
    }

    pub fn config_path(&self) -> PathBuf {
        self.cfg_dir.join("state.json")
    }

    pub fn load(&self) -> Result<LauncherConfig, ConfigError> {
        let path = self.config_path();
        if !path.exists() {
            return Ok(LauncherConfig::default());
        }
        let content = std::fs::read_to_string(path)?;
        let cfg = serde_json::from_str(&content)?;
        Ok(cfg)
    }

    pub fn save(&self, config: &LauncherConfig) -> Result<(), ConfigError> {
        let path = self.config_path();
        let content = serde_json::to_string_pretty(config)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn test_config_save_load() {
        let temp_dir = tempdir().unwrap();
        
        let manager = ConfigManager::with_dir(temp_dir.path().to_path_buf()).unwrap();
        
        // Should return default if not exists
        let auto_loaded = manager.load().unwrap();
        assert!(auto_loaded.games.is_empty());

        // Save
        let mut cfg = LauncherConfig::default();
        cfg.games.insert("Gothic2NotR".to_string(), GameState {
            install_path: Some(PathBuf::from("/test/gothic")),
            detected: true,
        });
        cfg.active_engine = Some("v1.0.4".to_string());
        
        manager.save(&cfg).unwrap();
        
        // Reload and verify
        let reloaded = manager.load().unwrap();
        let game = reloaded.games.get("Gothic2NotR").unwrap();
        assert_eq!(game.install_path, Some(PathBuf::from("/test/gothic")));
        assert!(game.detected);
        assert_eq!(reloaded.active_engine, Some("v1.0.4".to_string()));
        assert!(reloaded.active_profile.is_none());
    }

    #[test]
    fn test_config_file_is_json() {
        let temp_dir = tempdir().unwrap();
        let manager = ConfigManager::with_dir(temp_dir.path().to_path_buf()).unwrap();
        
        let cfg = LauncherConfig::default();
        manager.save(&cfg).unwrap();
        
        let content = std::fs::read_to_string(manager.config_path()).unwrap();
        // Should be valid JSON
        let _: serde_json::Value = serde_json::from_str(&content).unwrap();
        // File should be named state.json
        assert!(manager.config_path().file_name().unwrap().to_str().unwrap() == "state.json");
    }
}
