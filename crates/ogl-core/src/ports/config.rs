use async_trait::async_trait;
use crate::domain::config::LauncherConfig;
use crate::errors::CoreError;

#[async_trait]
pub trait ConfigStore: Send + Sync {
    async fn load(&self) -> Result<LauncherConfig, CoreError>;
    async fn save(&self, config: &LauncherConfig) -> Result<(), CoreError>;
}
