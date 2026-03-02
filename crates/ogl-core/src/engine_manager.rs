use std::path::PathBuf;
use thiserror::Error;
use dirs::data_local_dir;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("Failed accessing engine directory: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Data directory not found")]
    NoDataDir,
}

#[derive(Debug, Clone)]
pub struct EngineVersion {
    pub version: String,
    pub executable_path: PathBuf,
}

pub struct EngineManager {
    engines_dir: PathBuf,
}

impl EngineManager {
    pub fn new() -> Result<Self, EngineError> {
        let base_dir = data_local_dir().ok_or(EngineError::NoDataDir)?;
        let engines_dir = base_dir.join("OpenGothicLauncher").join("engines");
        if !engines_dir.exists() {
            std::fs::create_dir_all(&engines_dir)?;
        }
        Ok(Self { engines_dir })
    }

    pub fn engines_dir(&self) -> &PathBuf {
        &self.engines_dir
    }

    pub fn list_installed(&self) -> Result<Vec<EngineVersion>, EngineError> {
        let mut versions = Vec::new();
        for entry in std::fs::read_dir(&self.engines_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let name = entry.file_name().to_string_lossy().to_string();
                let executable_path = entry.path().join(if cfg!(windows) { "OpenGothic.exe" } else { "OpenGothic" });
                if executable_path.exists() {
                    versions.push(EngineVersion {
                        version: name,
                        executable_path,
                    });
                }
            }
        }
        Ok(versions)
    }
}
