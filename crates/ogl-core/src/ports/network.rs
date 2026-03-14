use std::path::Path;
use async_trait::async_trait;
use crate::domain::engine::EngineRelease;
use crate::errors::CoreError;

pub type DownloadProgress = Box<dyn FnMut(u64, u64) + Send>;

#[async_trait]
pub trait ReleaseProvider: Send + Sync {
    async fn latest_release(&self) -> Result<EngineRelease, CoreError>;
    async fn list_releases(&self) -> Result<Vec<EngineRelease>, CoreError>;
}

#[async_trait]
pub trait EngineDownloader: Send + Sync {
    async fn download(&self, url: &str, dest: &Path, progress: Option<DownloadProgress>) -> Result<(), CoreError>;
}

#[async_trait]
pub trait ArchiveExtractor: Send + Sync {
    async fn extract_zip(&self, archive_path: &Path, dest_dir: &Path) -> Result<(), CoreError>;
}
