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

pub async fn fetch_latest_release(custom_url: Option<&str>) -> Result<GitHubRelease, ReleaseError> {
    let url = custom_url.unwrap_or("https://api.github.com/repos/Try/OpenGothic/releases/latest");
    
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

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;

    #[tokio::test]
    async fn test_fetch_latest_release() {
        let mut server = Server::new_async().await;
        
        let mock_body = r#"{
            "tag_name": "v1.0.4",
            "name": "OpenGothic v1.0.4",
            "assets": [
                {
                    "name": "OpenGothic-win64.zip",
                    "browser_download_url": "https://example.com/download.zip",
                    "size": 123456
                }
            ]
        }"#;

        let mock = server.mock("GET", "/releases/latest")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_body)
            .create_async()
            .await;

        let url = format!("{}/releases/latest", server.url());
        let release = fetch_latest_release(Some(&url)).await.unwrap();

        assert_eq!(release.tag_name, "v1.0.4");
        assert_eq!(release.name, "OpenGothic v1.0.4");
        assert_eq!(release.assets.len(), 1);
        assert_eq!(release.assets[0].name, "OpenGothic-win64.zip");
        assert_eq!(release.assets[0].size, 123456);

        mock.assert_async().await;
    }
}
