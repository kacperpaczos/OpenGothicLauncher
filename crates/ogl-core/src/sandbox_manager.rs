use std::path::{Path, PathBuf};
use thiserror::Error;
use crate::app_dirs::AppDirs;

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("App directories error: {0}")]
    DirsError(#[from] crate::app_dirs::AppDirsError),
}

pub struct SandboxManager {
    sandboxes_dir: PathBuf,
}

impl SandboxManager {
    pub fn new() -> Result<Self, SandboxError> {
        let app_dirs = AppDirs::new()?;
        let sandboxes_dir = app_dirs.sandboxes_dir();
        Self::with_dir(sandboxes_dir)
    }

    pub fn with_dir(sandboxes_dir: PathBuf) -> Result<Self, SandboxError> {
        if !sandboxes_dir.exists() {
            std::fs::create_dir_all(&sandboxes_dir)?;
        }
        Ok(Self { sandboxes_dir })
    }

    /// Prepares a sandbox runtime environment for a specific profile
    pub fn prepare_sandbox(&self, profile_name: &str, _gothic_root: &Path) -> Result<PathBuf, SandboxError> {
        let sandbox_path = self.sandboxes_dir.join(profile_name);
        if !sandbox_path.exists() {
            std::fs::create_dir_all(&sandbox_path)?;
        }
        
        // MVP: The sandbox is just an isolated working directory.
        // In the future, this can symlink Data/, Saves/, or System/Gothic.ini etc.
        
        Ok(sandbox_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_prepare_sandbox() {
        let temp_dir = tempdir().unwrap();
        
        let manager = SandboxManager::with_dir(temp_dir.path().to_path_buf()).unwrap();
        let dummy_root = PathBuf::from("/dummy/gothic");
        
        let sandbox_path = manager.prepare_sandbox("MainProfile", &dummy_root).unwrap();
        
        assert!(sandbox_path.exists());
        assert!(sandbox_path.ends_with("MainProfile"));
    }
}
