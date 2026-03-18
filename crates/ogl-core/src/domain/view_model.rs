use serde::{Deserialize, Serialize};

use crate::domain::config::LauncherConfig;
use crate::domain::engine::{EngineRelease, EngineVersion};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressPayload {
    pub received: u64,
    pub total: u64,
    pub percentage: f64,
}

impl ProgressPayload {
    pub fn new(received: u64, total: u64) -> Self {
        let percentage = if total > 0 {
            (received as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        Self {
            received,
            total,
            percentage,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppViewModel {
    pub config: LauncherConfig,
    pub installed_engines: Vec<EngineVersion>,
    pub available_releases: Vec<EngineRelease>,
    pub background_task: Option<ProgressPayload>,
}

impl AppViewModel {
    pub fn new(
        config: LauncherConfig, 
        installed: Vec<EngineVersion>, 
        available: Vec<EngineRelease>
    ) -> Self {
        Self {
            config,
            installed_engines: installed,
            available_releases: available,
            background_task: None,
        }
    }
}
