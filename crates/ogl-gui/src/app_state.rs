use std::cell::RefCell;
use std::rc::Rc;

use ogl_core::install_detector::GothicGame;
use ogl_core::config_manager::{LauncherConfig, GameState, ConfigManager};
use ogl_core::engine_manager::{EngineManager, EngineVersion};

/// Runtime (non-persisted) state of the GUI application.
///
/// This is the ViewModel in MVVM — it wraps the persisted `LauncherConfig`
/// and adds transient UI state like selection, progress, and errors.
#[derive(Debug)]
pub struct AppState {
    /// Persisted config (written to ~/.config/OpenGothicLauncher/state.json)
    pub config: LauncherConfig,
    /// Which game is currently selected in the sidebar.
    pub selected_game: GothicGame,
    /// True while a background detection scan is running.
    pub detection_running: bool,
    /// Download progress 0.0..1.0 while an engine is being downloaded, None otherwise.
    pub download_progress: Option<f64>,
    /// List of locally installed OpenGothic engine versions.
    pub installed_engines: Vec<EngineVersion>,
    /// Transient error message to display in the UI.
    pub error_message: Option<String>,
}

impl AppState {
    /// Create a new AppState by loading persisted config from disk.
    pub fn load() -> Self {
        let config = ConfigManager::new()
            .and_then(|mgr| mgr.load())
            .unwrap_or_default();

        let installed_engines = EngineManager::new()
            .and_then(|mgr| mgr.list_installed())
            .unwrap_or_default();

        Self {
            config,
            selected_game: GothicGame::Gothic2NotR,
            detection_running: false,
            download_progress: None,
            installed_engines,
            error_message: None,
        }
    }

    /// Persist the current config to disk.
    pub fn save(&self) {
        if let Ok(mgr) = ConfigManager::new() {
            let _ = mgr.save(&self.config);
        }
    }

    /// Get the persisted GameState for the currently selected game.
    pub fn current_game_state(&self) -> GameState {
        let key = format!("{:?}", self.selected_game);
        self.config.games.get(&key).cloned().unwrap_or_default()
    }

    /// Update the persisted GameState for the currently selected game.
    pub fn set_current_game_state(&mut self, state: GameState) {
        let key = format!("{:?}", self.selected_game);
        self.config.games.insert(key, state);
    }

    /// Convenience: list of games shown in the sidebar.
    pub fn sidebar_games() -> Vec<GothicGame> {
        vec![
            GothicGame::Gothic1,
            GothicGame::Gothic2NotR,
            GothicGame::ChroniclesOfMyrtana,
            GothicGame::Gothic3,
        ]
    }
}

/// Shared, mutable reference to AppState used across GTK widgets.
pub type SharedState = Rc<RefCell<AppState>>;

/// Create a new shared AppState.
pub fn new_shared_state() -> SharedState {
    Rc::new(RefCell::new(AppState::load()))
}
