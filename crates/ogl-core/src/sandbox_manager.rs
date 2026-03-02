use std::path::{Path, PathBuf};
use thiserror::Error;
use dirs::data_local_dir;

#[derive(Debug, Error)]
pub enum SandboxError {
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Runtime data directory not found")]
    NoDataDir,
}

pub struct SandboxManager {
    sandboxes_dir: PathBuf,
}

impl SandboxManager {
    pub fn new() -> Result<Self, SandboxError> {
        let base_dir = data_local_dir().ok_or(SandboxError::NoDataDir)?;
        let sandboxes_dir = base_dir.join("OpenGothicLauncher").join("sandboxes");
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
