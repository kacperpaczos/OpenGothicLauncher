use std::path::{Path, PathBuf};
use std::process::Stdio;
use thiserror::Error;
use tokio::process::Command;
use tracing::{info, error};

#[derive(Debug, Error)]
pub enum ExecutorError {
    #[error("Failed to execute OpenGothic: {0}")]
    ProcessError(#[from] std::io::Error),
    #[error("OpenGothic exited with an error code: {0}")]
    ExitError(i32),
}

pub struct Executor {
    executable_path: PathBuf,
}

impl Executor {
    pub fn new<P: AsRef<Path>>(executable_path: P) -> Self {
        Self {
            executable_path: executable_path.as_ref().to_path_buf(),
        }
    }

    pub async fn launch(&self, gothic_root: &Path, mods: &[String]) -> Result<(), ExecutorError> {
        let mut command = Command::new(&self.executable_path);

        command.arg("--game").arg(gothic_root);
        
        for m in mods {
            command.arg("--mod").arg(m);
        }

        command.stdout(Stdio::piped())
               .stderr(Stdio::piped());

        info!("Launching OpenGothic: {:?}", command);

        let mut child = command.spawn()?;

        // Capture stdout/stderr via tokio (optional detailed implementation)
        // For now we just wait on it.

        let status = child.wait().await?;
        
        if !status.success() {
            let code = status.code().unwrap_or(-1);
            error!("OpenGothic exited with code {}", code);
            return Err(ExecutorError::ExitError(code));
        }

        info!("OpenGothic finished successfully");
        Ok(())
    }
}
