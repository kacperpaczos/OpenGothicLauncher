use async_trait::async_trait;
use crate::domain::launch::GameLaunch;
use crate::errors::CoreError;

#[async_trait]
pub trait GameProcessRunner: Send + Sync {
    async fn launch(&self, launch: &GameLaunch) -> Result<(), CoreError>;
}
