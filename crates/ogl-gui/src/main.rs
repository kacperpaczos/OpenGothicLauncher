mod app_state;
mod runtime;
mod window;
mod sidebar;
mod game_panel;
mod engine_window;
mod view_models;

use gtk4::prelude::*;
use gtk4::Application;
use std::sync::Arc;
use std::sync::Mutex;

use ogl_core::LauncherService;
use ogl_core::ports::AppPaths;
use ogl_infra::{
    TomlConfigStore, StdAppPaths, StdFileSystem, StdInstallDetector, StdModFilesProvider,
    StdPlatformProvider, ZipArchiveExtractor,
};
use ogl_network::{ReqwestDownloader, ReqwestReleaseProvider};
use ogl_executor::TokioGameRunner;
use crate::view_models::{AppUiState, AppViewModel, GamePanelViewModel};
use crate::app_state::AppContext;


const APP_ID: &str = "com.github.paczos.OpenGothicLauncher";

fn main() {
    // Initialize tracing for debug logs to both stdout and a file
    let config_dir = StdAppPaths::new()
        .map(|p| p.config_dir().join("logs"))
        .unwrap_or_else(|_| std::path::PathBuf::from("./logs"));
        
    std::fs::create_dir_all(&config_dir).unwrap_or_default();
    
    let file_appender = tracing_appender::rolling::daily(config_dir, "ogl-gui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(
            // Write to stdout and the rotating file
            tracing_subscriber::fmt::writer::MakeWriterExt::and(std::io::stdout, non_blocking)
        )
        .init();
        
    tracing::info!("Starting OpenGothicLauncher GUI...");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        let service = build_service().unwrap_or_else(|e| {
            panic!("Failed to build service: {}", e);
        });

        let ctx = Arc::new(AppContext::new(service));
        let (config, installed_engines) = runtime::background().block_on(async {
            let config = ctx.service.load_config().await.unwrap_or_default();
            let engines = ctx.service.list_installed_engines().await.unwrap_or_default();
            (config, engines)
        });

        let state = Arc::new(Mutex::new(AppUiState::new(config, installed_engines)));
        let app_vm = AppViewModel::new(ctx.clone(), state.clone());
        let game_panel_vm = GamePanelViewModel::new(app_vm.ctx(), app_vm.state());
        
        // Build and present the main window
        let win = window::build_window(app, &state, game_panel_vm);
        win.present();
    });

    app.run();
}

fn build_service() -> anyhow::Result<LauncherService> {
    let paths = Arc::new(StdAppPaths::new()?);
    let fs = Arc::new(StdFileSystem::new());
    let release_provider = Arc::new(ReqwestReleaseProvider::new());
    let downloader = Arc::new(ReqwestDownloader::new());
    let extractor = Arc::new(ZipArchiveExtractor::new());
    let config_store = Arc::new(TomlConfigStore::new(paths.clone(), fs.clone()));
    let install_detector = Arc::new(StdInstallDetector::new());
    let mod_files = Arc::new(StdModFilesProvider::new());
    let platform = Arc::new(StdPlatformProvider::new());
    let runner = Arc::new(TokioGameRunner::new());

    Ok(LauncherService::new(
        paths,
        fs,
        release_provider,
        downloader,
        extractor,
        config_store,
        install_detector,
        mod_files,
        platform,
        runner,
    ))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gui_compiles_and_harness_works() {
        // GTK startup tests require a display server (X11/Wayland). 
        // This test ensures that the module compiles correctly and cargo test works.
        assert_eq!(2 + 2, 4);
    }
}
