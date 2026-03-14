use crate::domain::engine::EnginePlatform;
use crate::errors::CoreError;

pub trait PlatformProvider: Send + Sync {
    fn current_platform(&self) -> Result<EnginePlatform, CoreError>;
}
