use std::path::PathBuf;
use ogl_core::CoreError;
use ogl_core::ports::AppPaths;
use directories::ProjectDirs;
use tracing::debug;

const QUALIFIER: &str = "com";
const ORGANIZATION: &str = "github";
const APPLICATION: &str = "OpenGothicLauncher";

#[derive(Debug, Clone)]
pub enum PathsMode {
    System,
    Portable(PathBuf),
}

#[derive(Clone)]
pub struct StdAppPaths {
    data_dir: PathBuf,
    config_dir: PathBuf,
}

impl StdAppPaths {
    pub fn new() -> Result<Self, CoreError> {
        let mode = Self::detect_mode()?;
        debug!("App paths mode: {:?}", mode);
        Self::from_mode(mode)
    }

    pub fn from_mode(mode: PathsMode) -> Result<Self, CoreError> {
        match mode {
            PathsMode::System => {
                let dirs = ProjectDirs::from(QUALIFIER, ORGANIZATION, APPLICATION)
                    .ok_or_else(|| CoreError::NotFound("Project directories not available".to_string()))?;
                Ok(Self {
                    data_dir: dirs.data_dir().to_path_buf(),
                    config_dir: dirs.config_dir().to_path_buf(),
                })
            }
            PathsMode::Portable(root) => Ok(Self {
                data_dir: root.join("data"),
                config_dir: root.join("config"),
            }),
        }
    }

    fn detect_mode() -> Result<PathsMode, CoreError> {
        if let Ok(root) = std::env::var("OGL_PORTABLE_DIR") {
            return Ok(PathsMode::Portable(PathBuf::from(root)));
        }
        if std::env::var("OGL_PORTABLE").ok().as_deref() == Some("1") {
            let exe = std::env::current_exe()
                .map_err(|e| CoreError::External(e.to_string()))?;
            if let Some(parent) = exe.parent() {
                return Ok(PathsMode::Portable(parent.to_path_buf()));
            }
        }
        Ok(PathsMode::System)
    }
}

impl AppPaths for StdAppPaths {
    fn data_dir(&self) -> PathBuf {
        self.data_dir.clone()
    }

    fn config_dir(&self) -> PathBuf {
        self.config_dir.clone()
    }

    fn engines_dir(&self) -> PathBuf {
        self.data_dir.join("engines")
    }

    fn sandboxes_dir(&self) -> PathBuf {
        self.data_dir.join("sandboxes")
    }
}
