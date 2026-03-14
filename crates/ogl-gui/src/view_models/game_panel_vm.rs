use std::sync::Arc;

use glib::MainContext;

use ogl_core::GothicGame;

use crate::app_state::SharedContext;
use crate::view_models::SharedUiState;
use crate::runtime;
use crate::view_models::EngineManagerViewModel;

#[derive(Clone)]
pub struct GamePanelViewModel {
    ctx: SharedContext,
    state: SharedUiState,
}

impl GamePanelViewModel {
    pub fn new(ctx: SharedContext, state: SharedUiState) -> Self {
        Self { ctx, state }
    }

    pub fn on_scan_clicked(&self) {
        let game = self.state.lock().unwrap().selected_game;
        {
            let mut s = self.state.lock().unwrap();
            s.detection_running = true;
            s.detection_progress = None;
            s.error_message = None;
        }

        let service = self.ctx.service.clone();
        let state = self.state.clone();
        let state_weak = Arc::downgrade(&state);

        let service_for_save = service.clone();
        runtime::background().spawn(async move {
            let state_weak_for_progress = state_weak.clone();
            let progress = std::sync::Arc::new(move |path: &std::path::Path| {
                let path_str = path.display().to_string();
                let state_weak = state_weak_for_progress.clone();
                MainContext::default().invoke(move || {
                    if let Some(state) = state_weak.upgrade() {
                        state.lock().unwrap().detection_progress = Some(path_str);
                    }
                });
            });

            let mut result = service.detect_installation(game, progress.clone()).await;
            if matches!(result, Ok(None)) {
                result = service.detect_installation_brute_force(game, progress).await;
            }

            let state_weak_for_result = state_weak.clone();
            MainContext::default().invoke(move || {
                if let Some(state) = state_weak_for_result.upgrade() {
                    let mut s = state.lock().unwrap();
                    s.detection_running = false;
                    s.detection_progress = None;

                    match result {
                        Ok(Some(install)) => {
                            let key = game.profile_id();
                            s.config.games.insert(key, ogl_core::GameState {
                                install_path: Some(install.root_path),
                                detected: true,
                            });
                            s.error_message = None;
                        }
                        Ok(None) => {
                            s.error_message = Some(format!("{} not found", game.display_name()));
                        }
                        Err(e) => {
                            s.error_message = Some(format!("Detection failed: {}", e));
                        }
                    }

                    let cfg = s.config.clone();
                    drop(s);
                    let service_for_save = service_for_save.clone();
                    runtime::background().spawn(async move {
                        let _ = service_for_save.save_config(&cfg).await;
                    });
                }
            });
        });
    }

    pub fn set_selected_game(&self, game: GothicGame) {
        let mut s = self.state.lock().unwrap();
        s.selected_game = game;
        s.error_message = None;
    }

    pub fn on_download_clicked(&self) {
        {
            self.state.lock().unwrap().download_progress = Some(0.0);
        }

        let service = self.ctx.service.clone();
        let state = self.state.clone();
        let state_weak = Arc::downgrade(&state);

        runtime::background().spawn(async move {
            let progress_state = state_weak.clone();
            let progress_cb = Box::new(move |current: u64, total: u64| {
                if total == 0 {
                    return;
                }
                let progress = (current as f64 / total as f64).clamp(0.0, 1.0);
                let state_weak = progress_state.clone();
                MainContext::default().invoke(move || {
                    if let Some(state) = state_weak.upgrade() {
                        state.lock().unwrap().download_progress = Some(progress);
                    }
                });
            });

            let result = service.install_open_gothic("latest", Some(progress_cb)).await;

            MainContext::default().invoke(move || {
                if let Some(state) = state_weak.upgrade() {
                    let mut s = state.lock().unwrap();
                    s.download_progress = None;
                    match result {
                        Ok(install) => {
                            s.config.active_engine = Some(install.version);
                            let service = service.clone();
                            let state_weak = state_weak.clone();
                            runtime::background().spawn(async move {
                                if let Ok(engines) = service.list_installed_engines().await {
                                    if let Some(state) = state_weak.upgrade() {
                                        state.lock().unwrap().installed_engines = engines;
                                    }
                                }
                                if let Ok(cfg) = service.load_config().await {
                                    if let Some(state) = state_weak.upgrade() {
                                        state.lock().unwrap().config = cfg;
                                    }
                                }
                            });
                            s.error_message = None;
                        }
                        Err(e) => {
                            s.error_message = Some(format!("Download failed: {}", e));
                        }
                    }
                }
            });
        });
    }

    pub fn on_launch_clicked(&self) {
        let game = self.state.lock().unwrap().selected_game;
        let service = self.ctx.service.clone();
        let state_weak = Arc::downgrade(&self.state);
        runtime::background().spawn(async move {
            let result = service.launch_profile(&game.profile_id()).await;
            if let Err(e) = result {
                MainContext::default().invoke(move || {
                    if let Some(state) = state_weak.upgrade() {
                        let mut s = state.lock().unwrap();
                        s.error_message = Some(format!("Launch failed: {}", e));
                    }
                });
            }
        });
    }

    pub fn save_config(&self) {
        let config = self.state.lock().unwrap().config.clone();
        let service = self.ctx.service.clone();
        runtime::background().spawn(async move {
            let _ = service.save_config(&config).await;
        });
    }

    pub fn engine_manager_view_model(&self) -> EngineManagerViewModel {
        EngineManagerViewModel::new(self.ctx.clone(), self.state.clone())
    }
}
