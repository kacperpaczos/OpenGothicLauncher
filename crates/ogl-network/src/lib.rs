pub mod releases;
pub mod downloads;

pub use releases::{fetch_latest_release, GitHubRelease, GitHubAsset, ReleaseError};
pub use downloads::{download_file, DownloadError};
