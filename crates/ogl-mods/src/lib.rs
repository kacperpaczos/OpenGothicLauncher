use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModError {
    #[error("Failed to read directory: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug, Clone)]
pub struct ModInfo {
    pub name: String,
    pub path: PathBuf,
    pub is_vdf: bool,
}

pub struct ModManager {
    gothic_root: PathBuf,
}

impl ModManager {
    pub fn new<P: AsRef<Path>>(gothic_root: P) -> Self {
        Self {
            gothic_root: gothic_root.as_ref().to_path_buf(),
        }
    }

    pub fn scan_mods(&self) -> Result<Vec<ModInfo>, ModError> {
        let mut mods = Vec::new();

        // Check Data/ directory for .vdf files
        let data_dir = self.gothic_root.join("Data");
        if data_dir.exists() {
            for entry in std::fs::read_dir(data_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_file() {
                    if let Some(ext) = path.extension() {
                        let ext_str = ext.to_string_lossy().to_lowercase();
                        if ext_str == "vdf" || ext_str == "mod" {
                            let name = path.file_stem()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            mods.push(ModInfo {
                                name,
                                path,
                                is_vdf: ext_str == "vdf",
                            });
                        }
                    }
                }
            }
        }

        // Ideally we also parse .ini files in System/ or other logic,
        // but this is the minimal viable detection.
        
        Ok(mods)
    }

    pub fn build_load_order(&self, enabled_mods: &[String]) -> Vec<String> {
        // In a real scenario, this resolves dependencies.
        // For MVP, just return them in the order provided.
        enabled_mods.to_vec()
    }
}
