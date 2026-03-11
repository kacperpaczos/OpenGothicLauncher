pub mod install_detector;
pub mod config_manager;
pub mod engine_manager;
pub mod sandbox_manager;

// Re-export the main public API surface
pub use install_detector::{
    detect, detect_brute_force,
    GothicGame, GothicInstall, DetectorError,
};
