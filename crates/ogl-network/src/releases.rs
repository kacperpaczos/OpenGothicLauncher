use serde::Deserialize;
use thiserror::Error;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

#[derive(Debug, Error)]
pub enum ReleaseError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("No release tags found in HTML")]
    NoTags,
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

fn parse_release_tags_from_html(html: &str) -> Vec<String> {
    let needle = "/Try/OpenGothic/releases/tag/";
    let mut tags = Vec::new();
    let mut search = html;

    while let Some(start) = search.find(needle) {
        let after = &search[start + needle.len()..];
        let end = after
            .find(|c: char| c == '"' || c == '\'' || c == '?' || c == '#')
            .unwrap_or(after.len());
        let tag = &after[..end];
        if !tag.is_empty() && !tags.iter().any(|t| t == tag) {
            tags.push(tag.to_string());
        }
        search = &after[end..];
    }

    tags
}

pub async fn fetch_latest_release_from_html(custom_url: Option<&str>) -> Result<GitHubRelease, ReleaseError> {
    let url = custom_url.unwrap_or("https://github.com/Try/OpenGothic/releases");

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("OpenGothicLauncher"));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()?;

    let html = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let tags = parse_release_tags_from_html(&html);
    let tag_name = tags.first().cloned().ok_or(ReleaseError::NoTags)?;

    let asset_names = ["opengothic_linux.zip", "opengothic_osx.zip", "opengothic_win.zip"];
    let assets = asset_names
        .into_iter()
        .map(|name| GitHubAsset {
            name: name.to_string(),
            browser_download_url: format!(
                "https://github.com/Try/OpenGothic/releases/download/{}/{}",
                tag_name, name
            ),
            size: 0,
        })
        .collect();

    Ok(GitHubRelease {
        tag_name: tag_name.clone(),
        name: tag_name,
        assets,
    })
}

pub async fn fetch_releases_from_html(custom_url: Option<&str>) -> Result<Vec<GitHubRelease>, ReleaseError> {
    let url = custom_url.unwrap_or("https://github.com/Try/OpenGothic/releases");

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("OpenGothicLauncher"));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .default_headers(headers)
        .build()?;

    let html = client
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let tags = parse_release_tags_from_html(&html);
    if tags.is_empty() {
        return Err(ReleaseError::NoTags);
    }

    let asset_names = ["opengothic_linux.zip", "opengothic_osx.zip", "opengothic_win.zip"];
    let releases = tags
        .into_iter()
        .map(|tag| GitHubRelease {
            tag_name: tag.clone(),
            name: tag.clone(),
            assets: asset_names
                .into_iter()
                .map(|name| GitHubAsset {
                    name: name.to_string(),
                    browser_download_url: format!(
                        "https://github.com/Try/OpenGothic/releases/download/{}/{}",
                        tag, name
                    ),
                    size: 0,
                })
                .collect(),
        })
        .collect();

    Ok(releases)
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

    #[test]
    fn test_parse_release_tags_from_html() {
        let html = r#"
            <a href="/Try/OpenGothic/releases/tag/opengothic-v1.0.3549">opengothic-v1.0.3549</a>
            <a href="/Try/OpenGothic/releases/tag/opengothic-v1.0.3548">opengothic-v1.0.3548</a>
        "#;

        let tags = parse_release_tags_from_html(html);
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0], "opengothic-v1.0.3549");
        assert_eq!(tags[1], "opengothic-v1.0.3548");
    }

    #[test]
    fn test_parse_release_tags_ignores_queries() {
        let html = r#"
            <a href="/Try/OpenGothic/releases/tag/opengothic-v1.0.9999?foo=bar">opengothic-v1.0.9999</a>
        "#;
        let tags = parse_release_tags_from_html(html);
        assert_eq!(tags, vec!["opengothic-v1.0.9999"]);
    }
}
