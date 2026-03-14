use std::path::PathBuf;

pub trait AppPaths: Send + Sync {
    fn data_dir(&self) -> PathBuf;
    fn config_dir(&self) -> PathBuf;
    fn engines_dir(&self) -> PathBuf;
    fn sandboxes_dir(&self) -> PathBuf;
}
