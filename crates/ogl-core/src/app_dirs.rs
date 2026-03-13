use std::path::PathBuf;
use thiserror::Error;
use dirs::home_dir;

const APP_DIR_NAME: &str = "OpenGothicLauncher";

#[derive(Debug, Error)]
pub enum AppDirsError {
    #[error("Data directory not found")]
    NoDataDir,
    #[error("Config directory not found")]
    NoConfigDir,
}

#[derive(Debug, Clone)]
pub struct AppDirs {
    data_dir: PathBuf,
    config_dir: PathBuf,
}

impl AppDirs {
    pub fn new() -> Result<Self, AppDirsError> {
        let home = home_dir().ok_or(AppDirsError::NoDataDir)?;
        let base = home.join(format!(".{}", APP_DIR_NAME));

        Ok(Self {
            data_dir: base.join("data"),
            config_dir: base.join("config"),
        })
    }

    pub fn data_dir(&self) -> &PathBuf {
        &self.data_dir
    }

    pub fn config_dir(&self) -> &PathBuf {
        &self.config_dir
    }

    pub fn engines_dir(&self) -> PathBuf {
        self.data_dir.join("engines")
    }

    pub fn sandboxes_dir(&self) -> PathBuf {
        self.data_dir.join("sandboxes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_app_dirs_from_home() {
        let temp = tempdir().unwrap();
        let original = std::env::var("HOME").ok();
        std::env::set_var("HOME", temp.path());

        let dirs = AppDirs::new().unwrap();
        assert!(dirs.data_dir().ends_with(".OpenGothicLauncher/data"));
        assert!(dirs.config_dir().ends_with(".OpenGothicLauncher/config"));

        if let Some(value) = original {
            std::env::set_var("HOME", value);
        }
    }
}
