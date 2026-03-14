use ogl_core::domain::engine::EnginePlatform;
use ogl_core::CoreError;
use ogl_core::ports::PlatformProvider;

#[derive(Clone, Default)]
pub struct StdPlatformProvider;

impl StdPlatformProvider {
    pub fn new() -> Self {
        Self
    }
}

impl PlatformProvider for StdPlatformProvider {
    fn current_platform(&self) -> Result<EnginePlatform, CoreError> {
        if cfg!(target_os = "linux") {
            Ok(EnginePlatform::Linux)
        } else if cfg!(target_os = "windows") {
            Ok(EnginePlatform::Windows)
        } else if cfg!(target_os = "macos") {
            Ok(EnginePlatform::MacOS)
        } else {
            Err(CoreError::UnsupportedPlatform)
        }
    }
}
