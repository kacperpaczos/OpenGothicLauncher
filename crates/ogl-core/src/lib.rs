pub mod domain;
pub mod ports;
pub mod services;

mod errors;

pub use crate::errors::{AppError, CoreError};
pub use crate::domain::{
    config::{LauncherConfig, GameState, Profile, ModboxConfig},
    engine::{EngineRelease, EngineAsset, EngineVersion, EngineInstall, EnginePlatform},
    install::{GothicGame, GothicInstall, GameMetadata},
    launch::GameLaunch,
    mods::{ModInfo, ModManager},
};
pub use crate::services::LauncherService;
