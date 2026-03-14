use std::sync::Arc;
use ogl_core::LauncherService;

/// Global app context: shared service and cross-cutting resources.
#[derive(Clone)]
pub struct AppContext {
    pub service: LauncherService,
}

impl AppContext {
    pub fn new(service: LauncherService) -> Self {
        Self { service }
    }
}

pub type SharedContext = Arc<AppContext>;
