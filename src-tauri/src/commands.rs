use tauri::{State, AppHandle, Emitter};
use ogl_core::{GothicGame, LauncherConfig, GameState};
use ogl_core::domain::view_model::{AppViewModel, ProgressPayload};
use std::path::PathBuf;
use std::sync::Arc;
use crate::AppState;

pub async fn broadcast_state(app: &AppHandle, state: &State<'_, AppState>) -> Result<(), String> {
    let config = state.launcher_service.load_config().await.map_err(|e| e.to_string())?;
    let installed = state.launcher_service.list_installed_engines().await.map_err(|e| e.to_string())?;
    let available = state.launcher_service.list_available_releases().await.unwrap_or_default();
    
    let view_model = AppViewModel::new(config, installed, available);
    app.emit("state_updated", view_model).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn get_state(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn save_config(app: AppHandle, state: State<'_, AppState>, config: LauncherConfig) -> Result<(), String> {
    state.launcher_service.save_config(&config).await.map_err(|e| e.to_string())?;
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn list_installed_engines(state: State<'_, AppState>) -> Result<Vec<ogl_core::EngineVersion>, String> {
    state.launcher_service.list_installed_engines().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_available_releases(state: State<'_, AppState>) -> Result<Vec<ogl_core::EngineRelease>, String> {
    state.launcher_service.list_available_releases().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn launch_game(state: State<'_, AppState>, game: GothicGame) -> Result<(), String> {
    state.launcher_service.launch_profile(&game.profile_id()).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn scan_for_games(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let installs = state.launcher_service.scan_for_installations(Arc::new(|_| {})).await.map_err(|e| e.to_string())?;
    let mut config = state.launcher_service.load_config().await.map_err(|e| e.to_string())?;
    
    for install in installs {
        config.games.insert(install.game.profile_id(), GameState {
            install_path: Some(install.root_path),
            detected: true,
        });
    }
    
    state.launcher_service.save_config(&config).await.map_err(|e| e.to_string())?;
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn manual_select_game_path(app: AppHandle, state: State<'_, AppState>, game: GothicGame, path: String) -> Result<(), String> {
    let mut config = state.launcher_service.load_config().await.map_err(|e| e.to_string())?;
    config.games.insert(game.profile_id(), GameState {
        install_path: Some(PathBuf::from(path)),
        detected: true,
    });
    state.launcher_service.save_config(&config).await.map_err(|e| e.to_string())?;
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn download_engine(app: AppHandle, state: State<'_, AppState>, version: String) -> Result<(), String> {
    let progress_callback = {
        let app = app.clone();
        Box::new(move |received, total| {
            let percentage = if total > 0 { (received as f64 / total as f64) * 100.0 } else { 0.0 };
            let _ = app.emit("download-progress", ProgressPayload { received, total, percentage });
        })
    };

    state.launcher_service
        .install_open_gothic(&version, Some(progress_callback))
        .await
        .map_err(|e| e.to_string())?;
        
    broadcast_state(&app, &state).await
}
