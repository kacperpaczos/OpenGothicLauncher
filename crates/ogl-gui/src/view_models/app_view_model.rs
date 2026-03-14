use std::sync::{Arc, Mutex};

use ogl_core::{EngineRelease, EngineVersion, GameState, GothicGame, LauncherConfig};

use crate::app_state::SharedContext;

/// UI-visible state owned by ViewModels.
#[derive(Debug)]
pub struct AppUiState {
    pub config: LauncherConfig,
    pub selected_game: GothicGame,
    pub detection_running: bool,
    pub detection_progress: Option<String>,
    pub download_progress: Option<f64>,
    pub installed_engines: Vec<EngineVersion>,
    pub available_releases: Vec<EngineRelease>,
    pub error_message: Option<String>,
}

impl AppUiState {
    pub fn new(config: LauncherConfig, installed_engines: Vec<EngineVersion>) -> Self {
        Self {
            config,
            selected_game: GothicGame::Gothic2NotR,
            detection_running: false,
            detection_progress: None,
            download_progress: None,
            installed_engines,
            available_releases: Vec::new(),
            error_message: None,
        }
    }

    pub fn current_game_state(&self) -> GameState {
        let key = format!("{:?}", self.selected_game);
        self.config.games.get(&key).cloned().unwrap_or_default()
    }

    pub fn set_current_game_state(&mut self, state: GameState) {
        let key = format!("{:?}", self.selected_game);
        self.config.games.insert(key, state);
    }

    pub fn sidebar_games() -> Vec<GothicGame> {
        vec![
            GothicGame::Gothic1,
            GothicGame::Gothic2,
            GothicGame::Gothic2NotR,
            GothicGame::ChroniclesOfMyrtana,
            GothicGame::Gothic3,
        ]
    }
}

pub type SharedUiState = Arc<Mutex<AppUiState>>;

#[derive(Clone)]
pub struct AppViewModel {
    ctx: SharedContext,
    state: SharedUiState,
}

impl AppViewModel {
    pub fn new(ctx: SharedContext, state: SharedUiState) -> Self {
        Self { ctx, state }
    }

    pub fn ctx(&self) -> SharedContext {
        self.ctx.clone()
    }

    pub fn state(&self) -> SharedUiState {
        self.state.clone()
    }
}
