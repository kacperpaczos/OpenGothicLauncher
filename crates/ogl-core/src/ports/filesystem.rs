use std::path::{Path, PathBuf};
use async_trait::async_trait;
use crate::errors::CoreError;

#[async_trait]
pub trait FileSystem: Send + Sync {
    async fn exists(&self, path: &Path) -> bool;
    async fn is_file(&self, path: &Path) -> bool;
    async fn is_dir(&self, path: &Path) -> bool;
    async fn read_dir(&self, path: &Path) -> Result<Vec<PathBuf>, CoreError>;
    async fn create_dir_all(&self, path: &Path) -> Result<(), CoreError>;
    async fn remove_file(&self, path: &Path) -> Result<(), CoreError>;
    async fn remove_dir_all(&self, path: &Path) -> Result<(), CoreError>;
    async fn read_to_string(&self, path: &Path) -> Result<String, CoreError>;
    async fn write_string(&self, path: &Path, contents: &str) -> Result<(), CoreError>;
}
