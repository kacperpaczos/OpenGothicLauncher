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


/// Parse actual asset download links from the GitHub expanded_assets HTML fragment.
/// Returns Vec of (filename, full_download_url).
fn parse_assets_from_expanded_html(html: &str, _tag: &str) -> Vec<GitHubAsset> {
    let needle = "/Try/OpenGothic/releases/download/";
    let mut assets = Vec::new();
    let mut search = html;

    while let Some(start) = search.find(needle) {
        let after = &search[start..];
        let end = after
            .find(|c: char| c == '"' || c == '\'' || c == '?' || c == '#' || c == ' ')
            .unwrap_or(after.len());
        let rel_path = &after[..end];
        // rel_path = /Try/OpenGothic/releases/download/{tag}/{filename}
        let full_url = format!("https://github.com{}", rel_path);
        let filename = rel_path.rsplit('/').next().unwrap_or("").to_string();
        
        // Skip source archives and empty filenames, only include actual engine zips
        if !filename.is_empty()
            && !filename.ends_with(".tar.gz")
            && !assets.iter().any(|a: &GitHubAsset| a.name == filename)
        {
            assets.push(GitHubAsset {
                name: filename,
                browser_download_url: full_url,
                size: 0,
            });
        }
        search = &after[end..];
    }

    assets
}

/// Fetches all releases by scraping the GitHub HTML releases page for tags,
/// then querying expanded_assets/{tag} for each tag to discover real download links.
pub async fn fetch_releases_from_html(custom_url: Option<&str>) -> Result<Vec<GitHubRelease>, ReleaseError> {
    let base_url = custom_url.unwrap_or("https://github.com/Try/OpenGothic/releases");

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("OpenGothicLauncher"));

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(15))
        .default_headers(headers)
        .build()?;

    // Step 1: Fetch the releases page and extract all tags
    let html = client
        .get(base_url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    let tags = parse_release_tags_from_html(&html);
    if tags.is_empty() {
        return Err(ReleaseError::NoTags);
    }

    // Step 2: For each tag, fetch expanded_assets to get real download links
    let mut releases = Vec::new();
    for tag in &tags {
        let assets_url = format!(
            "https://github.com/Try/OpenGothic/releases/expanded_assets/{}",
            tag
        );
        match client.get(&assets_url).send().await {
            Ok(resp) => {
                if let Ok(assets_html) = resp.text().await {
                    let assets = parse_assets_from_expanded_html(&assets_html, tag);
                    releases.push(GitHubRelease {
                        tag_name: tag.clone(),
                        name: tag.clone(),
                        assets,
                    });
                }
            }
            Err(_) => {
                // If we can't fetch assets for a tag, skip it
                continue;
            }
        }
    }

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
