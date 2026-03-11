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
        Self::with_dir(engines_dir)
    }

    pub fn with_dir(engines_dir: PathBuf) -> Result<Self, EngineError> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_list_installed_engines() {
        let temp_dir = tempdir().unwrap();
        let manager = EngineManager::with_dir(temp_dir.path().to_path_buf()).unwrap();
        assert!(manager.list_installed().unwrap().is_empty());
        
        let engine_ver_dir = temp_dir.path().join("v1.0.4");
        fs::create_dir_all(&engine_ver_dir).unwrap();
        let exe_name = if cfg!(windows) { "OpenGothic.exe" } else { "OpenGothic" };
        
        // Exists dir but no exe
        assert!(manager.list_installed().unwrap().is_empty());
        
        // Exists dir and exe
        fs::write(engine_ver_dir.join(exe_name), "dummy exe file").unwrap();
        
        let installed = manager.list_installed().unwrap();
        assert_eq!(installed.len(), 1);
        assert_eq!(installed[0].version, "v1.0.4");
    }
}
