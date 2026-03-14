use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ModError {
    #[error("Invalid mod path: {0}")]
    InvalidPath(String),
}

#[derive(Debug, Clone)]
pub struct ModInfo {
    pub name: String,
    pub path: PathBuf,
    pub is_vdf: bool,
}

pub struct ModManager;

impl ModManager {
    pub fn from_paths<I>(paths: I) -> Result<Vec<ModInfo>, ModError>
    where
        I: IntoIterator<Item = PathBuf>,
    {
        let mut mods = Vec::new();

        for path in paths {
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy().to_lowercase();
                if ext_str == "vdf" || ext_str == "mod" {
                    let name = path
                        .file_stem()
                        .ok_or_else(|| ModError::InvalidPath(path.to_string_lossy().to_string()))?
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

        Ok(mods)
    }

    pub fn build_load_order(enabled_mods: &[String]) -> Vec<String> {
        enabled_mods.to_vec()
    }
}
