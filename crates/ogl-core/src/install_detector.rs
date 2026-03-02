use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DetectorError {
    #[error("Gothic installation not found in typical locations")]
    NotFound,
    #[error("I/O Error during detection: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Failed to read configuration: {0}")]
    ConfigError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GothicInstall {
    pub root_path: PathBuf,
    pub executable_path: PathBuf,
    pub data_dir: PathBuf,
}

/// Detects the classic Gothic/Gothic 2 installation.
pub fn detect_installation() -> Result<GothicInstall, DetectorError> {
    // Basic fallback logic; to be expanded with Windows Registry
    // and common Linux/macOS paths depending on the OS.
    
    // For now, let's just do a naive check in a few common paths,
    // or return NotFound.
    let common_paths = vec![
        PathBuf::from("C:\\Program Files (x86)\\Steam\\steamapps\\common\\Gothic II"),
        PathBuf::from("C:\\Gothic II"),
        dirs::home_dir().unwrap_or_default().join(".steam/steam/steamapps/common/Gothic II"),
        dirs::home_dir().unwrap_or_default().join(".local/share/Steam/steamapps/common/Gothic II"),
    ];

    for path in common_paths {
        if is_valid_gothic_root(&path) {
            return Ok(GothicInstall {
                executable_path: path.join("System").join("Gothic2.exe"),
                data_dir: path.join("Data"),
                root_path: path,
            });
        }
    }

    Err(DetectorError::NotFound)
}

fn is_valid_gothic_root(path: &Path) -> bool {
    // Check if Data/ and System/ exist
    let sys_exists = path.join("System").exists();
    let data_exists = path.join("Data").exists();
    sys_exists && data_exists
}
