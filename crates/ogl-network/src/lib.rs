pub mod releases;
pub mod downloads;

pub use releases::{fetch_latest_release, fetch_latest_release_from_html, GitHubRelease, GitHubAsset, ReleaseError};
pub use downloads::{download_file, DownloadError};

mod adapters;
pub use adapters::{ReqwestDownloader, ReqwestReleaseProvider};
