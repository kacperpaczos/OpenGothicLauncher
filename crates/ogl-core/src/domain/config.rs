use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Per-game persistent state (serialized in state.json).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GameState {
    /// Detected installation path on disk.
    pub install_path: Option<PathBuf>,
    /// Whether the game was successfully detected.
    pub detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LauncherConfig {
    /// Currently selected OpenGothic engine version tag (e.g. "v1.0.4").
    pub active_engine: Option<String>,
    /// Currently active sandbox/profile name.
    pub active_profile: Option<String>,
    /// Per-game detection state, keyed by game variant name.
    pub games: HashMap<String, GameState>,
}
