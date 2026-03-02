use serde::Deserialize;
use thiserror::Error;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

#[derive(Debug, Error)]
pub enum ReleaseError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("No valid assets found in release")]
    NoAssets,
}

#[derive(Debug, Deserialize)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: String,
    pub assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

pub async fn fetch_latest_release() -> Result<GitHubRelease, ReleaseError> {
    let url = "https://api.github.com/repos/Try/OpenGothic/releases/latest";
    
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("OpenGothicLauncher"));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()?;

    let release: GitHubRelease = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;

    Ok(release)
}
