pub mod install_detector;
pub mod config_manager;
pub mod engine_manager;
pub mod sandbox_manager;

// This will re-export core functionalities
pub use install_detector::{detect_installation, GothicInstall, DetectorError};
