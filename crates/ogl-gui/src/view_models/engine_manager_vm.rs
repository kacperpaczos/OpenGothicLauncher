use crate::app_state::SharedContext;
use crate::view_models::SharedUiState;
use crate::runtime;

#[derive(Clone)]
pub struct EngineManagerViewModel {
    ctx: SharedContext,
    state: SharedUiState,
}

impl EngineManagerViewModel {
    pub fn new(ctx: SharedContext, state: SharedUiState) -> Self {
        Self { ctx, state }
    }

    pub fn refresh(&self) {
        let service = self.ctx.service.clone();
        let state = self.state.clone();
        runtime::background().spawn(async move {
            let engines = service.list_installed_engines().await.ok();
            let cfg = service.load_config().await.ok();
            let releases = service.list_available_releases().await.ok();
            if let Some(engines) = engines {
                state.lock().unwrap().installed_engines = engines;
            }
            if let Some(cfg) = cfg {
                state.lock().unwrap().config = cfg;
            }
            if let Some(releases) = releases {
                state.lock().unwrap().available_releases = releases;
            }
        });
    }

    pub fn engines_dir_label(&self) -> String {
        self.ctx.service
            .engines_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|e| format!("Failed to resolve engines dir: {}", e))
    }
}
