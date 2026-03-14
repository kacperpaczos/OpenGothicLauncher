use std::sync::Arc;

use ogl_core::domain::config::LauncherConfig;
use ogl_core::CoreError;
use ogl_core::ports::{AppPaths, ConfigStore, FileSystem};
use async_trait::async_trait;
use ogl_assets::DEFAULT_CONFIG_TOML;
use tracing::debug;

#[derive(Clone)]
pub struct TomlConfigStore {
    paths: Arc<dyn AppPaths>,
    fs: Arc<dyn FileSystem>,
}

impl TomlConfigStore {
    pub fn new(paths: Arc<dyn AppPaths>, fs: Arc<dyn FileSystem>) -> Self {
        Self { paths, fs }
    }

    fn config_path(&self) -> std::path::PathBuf {
        self.paths.config_dir().join("state.toml")
    }
}

#[async_trait]
impl ConfigStore for TomlConfigStore {
    async fn load(&self) -> Result<LauncherConfig, CoreError> {
        let path = self.config_path();
        if !self.fs.exists(&path).await {
            let legacy_json = self.paths.config_dir().join("state.json");
            if self.fs.exists(&legacy_json).await {
                debug!("Migrating legacy config from {}", legacy_json.display());
                let content = self.fs.read_to_string(&legacy_json).await?;
                let cfg: LauncherConfig = serde_json::from_str(&content)
                    .map_err(|e| CoreError::External(e.to_string()))?;
                self.save(&cfg).await?;
                return Ok(cfg);
            }
            debug!("Config not found at {}, using embedded default", path.display());
            return toml::from_str(DEFAULT_CONFIG_TOML)
                .map_err(|e| CoreError::External(e.to_string()));
        }
        debug!("Loading config from {}", path.display());
        let content = self.fs.read_to_string(&path).await?;
        toml::from_str(&content).map_err(|e| CoreError::External(e.to_string()))
    }

    async fn save(&self, config: &LauncherConfig) -> Result<(), CoreError> {
        let cfg_dir = self.paths.config_dir();
        if !self.fs.exists(&cfg_dir).await {
            self.fs.create_dir_all(&cfg_dir).await?;
        }
        debug!("Saving config to {}", self.config_path().display());
        let content = toml::to_string_pretty(config)
            .map_err(|e| CoreError::External(e.to_string()))?;
        self.fs.write_string(&self.config_path(), &content).await?;
        Ok(())
    }
}
