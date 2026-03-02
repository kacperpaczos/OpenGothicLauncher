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
