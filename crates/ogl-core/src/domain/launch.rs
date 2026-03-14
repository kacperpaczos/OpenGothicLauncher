use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct GameLaunch {
    pub executable_path: PathBuf,
    pub gothic_root: PathBuf,
    pub mods: Vec<String>,
}
