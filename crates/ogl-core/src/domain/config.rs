use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

/// Per-game persistent state (serialized in state.json).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct GameState {
    /// Detected installation path on disk.
    pub install_path: Option<PathBuf>,
    /// Whether the game was successfully detected.
    pub detected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ThemeConfig {
    pub bg_color: String,
    pub panel_bg: String,
    pub sidebar_bg: String,
    pub accent_color: String,
    pub text_primary: String,
    pub text_secondary: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            bg_color: "hsl(0 0% 3.9%)".to_string(),
            panel_bg: "hsl(0 0% 9%)".to_string(),
            sidebar_bg: "hsl(0 0% 6%)".to_string(),
            accent_color: "hsl(24.6 95% 53.1%)".to_string(),
            text_primary: "hsl(0 0% 98%)".to_string(),
            text_secondary: "hsl(0 0% 63.9%)".to_string(),
        }
    }
}

/// Configuration for a modbox (Git-based mod management).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModboxConfig {
    /// List of mod identifiers or URLs.
    pub mods: Vec<String>,
}

/// A specific game profile combining a game version, engine version, and mods.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    /// Unique identifier for the profile.
    pub id: String,
    /// Human-readable name (e.g., "Vanilla", "L'Hiver").
    pub name: String,
    /// The base game for this profile.
    pub game: crate::GothicGame,
    /// Specific engine version tag to use.
    pub engine_version: Option<String>,
    /// Modbox configuration for this profile.
    pub modbox: ModboxConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LauncherConfig {
    /// Currently selected OpenGothic engine version tag (e.g. "v1.0.4").
    pub active_engine: Option<String>,
    /// Currently active profile ID.
    pub active_profile_id: Option<String>,
    /// Per-game detection state, keyed by game variant name.
    pub games: HashMap<String, GameState>,
    /// List of user-defined profiles.
    #[serde(default)]
    pub profiles: Vec<Profile>,
    /// Theme configuration driven by Rust.
    #[serde(default)]
    pub theme: ThemeConfig,
}

