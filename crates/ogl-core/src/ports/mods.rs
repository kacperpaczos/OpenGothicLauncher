use std::path::{Path, PathBuf};
use async_trait::async_trait;
use crate::domain::install::GothicGame;
use crate::errors::CoreError;

#[async_trait]
pub trait ModFilesProvider: Send + Sync {
    async fn list_mod_files(&self, game: GothicGame, gothic_root: &Path) -> Result<Vec<PathBuf>, CoreError>;
}
