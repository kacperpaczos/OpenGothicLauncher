mod commands;

use std::sync::Arc;
use ogl_core::LauncherService;
use ogl_infra::{
    TomlConfigStore, StdAppPaths, StdFileSystem, StdInstallDetector, StdModFilesProvider,
    StdPlatformProvider, ZipArchiveExtractor,
};
use ogl_network::{ReqwestDownloader, ReqwestReleaseProvider};
use ogl_executor::TokioGameRunner;
use tracing::info;

pub struct AppState {
    pub launcher_service: LauncherService,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    let file_appender = tracing_appender::rolling::never("logs", "log.txt");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .with_writer(non_blocking)
        .with_env_filter("info,opengothic_launcher=debug,ogl_core=debug,ogl_infra=debug,ogl_network=debug")
        .init();

    info!("Application starting up...");

    let service = build_service().expect("Failed to build LauncherService");
    let state = AppState { launcher_service: service };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            commands::get_state,
            commands::save_config,
            commands::list_installed_engines,
            commands::get_available_releases,
            commands::launch_game,
            commands::scan_for_games,
            commands::manual_select_game_path,
            commands::download_engine,
            commands::log_action,
            commands::delete_engine,
            commands::reinstall_engine
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
