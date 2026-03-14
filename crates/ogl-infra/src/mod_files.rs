use std::path::{Path, PathBuf};
use async_trait::async_trait;
use tokio::fs;

use ogl_core::domain::install::GothicGame;
use ogl_core::CoreError;
use ogl_core::ports::ModFilesProvider;

#[derive(Clone, Default)]
pub struct StdModFilesProvider;

impl StdModFilesProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ModFilesProvider for StdModFilesProvider {
    async fn list_mod_files(&self, _game: GothicGame, gothic_root: &Path) -> Result<Vec<PathBuf>, CoreError> {
        let data_dir = gothic_root.join("Data");
        if fs::metadata(&data_dir).await.is_err() {
            return Ok(Vec::new());
        }
        let mut entries = fs::read_dir(&data_dir).await.map_err(|e| CoreError::Io(e.to_string()))?;
        let mut paths = Vec::new();
        while let Ok(Some(entry)) = entries.next_entry().await {
            paths.push(entry.path());
        }
        Ok(paths)
    }
}
