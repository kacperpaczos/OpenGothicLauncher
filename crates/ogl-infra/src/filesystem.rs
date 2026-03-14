use std::path::{Path, PathBuf};
use async_trait::async_trait;
use tokio::fs;

use ogl_core::CoreError;
use ogl_core::ports::FileSystem;

#[derive(Clone, Default)]
pub struct StdFileSystem;

impl StdFileSystem {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl FileSystem for StdFileSystem {
    async fn exists(&self, path: &Path) -> bool {
        fs::metadata(path).await.is_ok()
    }

    async fn is_file(&self, path: &Path) -> bool {
        fs::metadata(path).await.map(|m| m.is_file()).unwrap_or(false)
    }

    async fn is_dir(&self, path: &Path) -> bool {
        fs::metadata(path).await.map(|m| m.is_dir()).unwrap_or(false)
    }

    async fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>, CoreError> {
        let mut entries = fs::read_dir(path).await.map_err(|e| CoreError::Io(e.to_string()))?;
        let mut paths = Vec::new();
        while let Ok(Some(entry)) = entries.next_entry().await {
            paths.push(entry.path());
        }
        Ok(paths)
    }

    async fn create_dir_all(&self, path: &Path) -> Result<(), CoreError> {
        fs::create_dir_all(path).await.map_err(|e| CoreError::Io(e.to_string()))
    }

    async fn remove_file(&self, path: &Path) -> Result<(), CoreError> {
        fs::remove_file(path).await.map_err(|e| CoreError::Io(e.to_string()))
    }

    async fn read_to_string(&self, path: &Path) -> Result<String, CoreError> {
        fs::read_to_string(path).await.map_err(|e| CoreError::Io(e.to_string()))
    }

    async fn write_string(&self, path: &Path, contents: &str) -> Result<(), CoreError> {
        fs::write(path, contents).await.map_err(|e| CoreError::Io(e.to_string()))
    }
}
