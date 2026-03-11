use std::path::Path;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use sha2::{Sha256, Digest};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

#[derive(Debug, Error)]
pub enum DownloadError {
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Hash mismatch: expected {expected}, got {actual}")]
    HashMismatch { expected: String, actual: String },
}

pub async fn download_file<P: AsRef<Path>>(
    url: &str,
    dest: P,
    expected_sha256: Option<&str>,
) -> Result<(), DownloadError> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("OpenGothicLauncher"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let mut response = client.get(url).send().await?.error_for_status()?;
    let mut file = tokio::fs::File::create(dest.as_ref()).await?;
    let mut hasher = Sha256::new();

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        hasher.update(&chunk);
    }
    file.flush().await?;

    if let Some(expected) = expected_sha256 {
        let result = hasher.finalize();
        let actual = hex::encode(result);
        if actual != expected {
            // Remove the corrupted file
            let _ = tokio::fs::remove_file(dest.as_ref()).await;
            return Err(DownloadError::HashMismatch {
                expected: expected.to_string(),
                actual,
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use sha2::{Sha256, Digest};
    use std::fs;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_download_file_success() {
        let mut server = Server::new_async().await;
        let file_content = b"fake engine archive data";
        
        let mock = server.mock("GET", "/download.zip")
            .with_status(200)
            .with_body(file_content)
            .create_async()
            .await;

        let temp_dir = tempdir().unwrap();
        let dest = temp_dir.path().join("test.zip");

        let url = format!("{}/download.zip", server.url());
        
        // Calculate expected hash
        let mut hasher = Sha256::new();
        hasher.update(file_content);
        let expected_hash = hex::encode(hasher.finalize());

        // Download with correct hash
        download_file(&url, &dest, Some(&expected_hash)).await.unwrap();
        
        assert!(dest.exists());
        let downloaded = fs::read(&dest).unwrap();
        assert_eq!(downloaded, file_content);

        mock.assert_async().await;
    }

    #[tokio::test]
    async fn test_download_file_hash_mismatch() {
        let mut server = Server::new_async().await;
        let file_content = b"corrupted data";
        
        server.mock("GET", "/corrupted.zip")
            .with_status(200)
            .with_body(file_content)
            .create_async()
            .await;

        let temp_dir = tempdir().unwrap();
        let dest = temp_dir.path().join("corrupted.zip");

        let url = format!("{}/corrupted.zip", server.url());
        
        // Use deliberately wrong hash
        let wrong_hash = "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890";

        let result = download_file(&url, &dest, Some(wrong_hash)).await;
        
        assert!(result.is_err());
        match result {
            Err(DownloadError::HashMismatch { expected, actual }) => {
                assert_eq!(expected, wrong_hash);
                assert_ne!(actual, wrong_hash);
            },
            _ => panic!("Expected HashMismatch error"),
        }
        
        // File should be deleted on corruption
        assert!(!dest.exists());
    }
}
