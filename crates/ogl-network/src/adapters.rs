use async_trait::async_trait;
use ogl_core::domain::engine::{EngineAsset, EngineRelease};
use ogl_core::CoreError;
use ogl_core::ports::{DownloadProgress, EngineDownloader, ReleaseProvider};
use tracing::debug;

use crate::downloads::download_file;
use crate::releases::{fetch_latest_release, fetch_releases_from_html, GitHubRelease};

#[derive(Clone, Default)]
pub struct ReqwestReleaseProvider;

impl ReqwestReleaseProvider {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ReleaseProvider for ReqwestReleaseProvider {
    async fn latest_release(&self) -> Result<EngineRelease, CoreError> {
        debug!("Fetching latest OpenGothic release metadata");
        let release = fetch_latest_release(None)
            .await
            .map_err(|e| CoreError::External(e.to_string()))?;
        Ok(map_release(release))
    }

    async fn list_releases(&self) -> Result<Vec<EngineRelease>, CoreError> {
        debug!("Fetching OpenGothic release list via HTML scraping");
        let releases = fetch_releases_from_html(None)
            .await
            .map_err(|e| CoreError::External(e.to_string()))?;
        Ok(releases.into_iter().map(map_release).collect())
    }
}

#[derive(Clone, Default)]
pub struct ReqwestDownloader;

impl ReqwestDownloader {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EngineDownloader for ReqwestDownloader {
    async fn download(&self, url: &str, dest: &std::path::Path, progress: Option<DownloadProgress>) -> Result<(), CoreError> {
        debug!("Downloading engine asset from {}", url);
        download_file(url, dest, None, progress)
            .await
            .map_err(|e| CoreError::External(e.to_string()))
    }
}

fn map_release(release: GitHubRelease) -> EngineRelease {
    EngineRelease {
        tag: release.tag_name.clone(),
        name: release.name,
        assets: release.assets.into_iter().map(map_asset).collect(),
    }
}

fn map_asset(asset: crate::releases::GitHubAsset) -> EngineAsset {
    EngineAsset {
        name: asset.name,
        download_url: asset.browser_download_url,
        size: asset.size,
    }
}
