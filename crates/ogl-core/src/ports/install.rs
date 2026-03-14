use std::path::Path;
use async_trait::async_trait;
use crate::domain::install::{GothicGame, GothicInstall};
use crate::errors::CoreError;

pub type DetectProgress = std::sync::Arc<dyn Fn(&Path) + Send + Sync>;

#[async_trait]
pub trait InstallDetector: Send + Sync {
    async fn detect(&self, game: GothicGame, on_progress: DetectProgress) -> Result<Option<GothicInstall>, CoreError>;
    async fn detect_brute_force(&self, game: GothicGame, on_progress: DetectProgress) -> Result<Option<GothicInstall>, CoreError>;
}
