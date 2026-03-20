use tauri::{State, AppHandle, Emitter};
use ogl_core::{GothicGame, LauncherConfig, GameState};
use ogl_core::domain::view_model::{AppViewModel, ProgressPayload};
use std::path::PathBuf;
use std::sync::Arc;
use crate::AppState;
use tracing::{info, error};

pub async fn broadcast_state(app: &AppHandle, state: &State<'_, AppState>) -> Result<(), String> {
    let config = state.launcher_service.load_config().await.map_err(|e| e.to_string())?;
    let installed = state.launcher_service.list_installed_engines().await.map_err(|e| e.to_string())?;
    let available = state.launcher_service.list_available_releases().await.unwrap_or_default();
    
    let view_model = AppViewModel::new(config, installed, available);
    info!("Broadcasting state update: {:?}", view_model.config);
    app.emit("state_updated", view_model).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn log_action(action: String, details: Option<String>) -> Result<(), String> {
    if let Some(d) = details {
        info!("UI Action: {} ({})", action, d);
    } else {
        info!("UI Action: {}", action);
    }
    Ok(())
}

#[tauri::command]
pub async fn get_state(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn save_config(app: AppHandle, state: State<'_, AppState>, config: LauncherConfig) -> Result<(), String> {
    state.launcher_service.save_config(&config).await.map_err(|e| {
        let msg = e.to_string();
        error!("Failed to save config: {}", msg);
        msg
    })?;
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn list_installed_engines(state: State<'_, AppState>) -> Result<Vec<ogl_core::EngineVersion>, String> {
    let engines = state.launcher_service.list_installed_engines().await.map_err(|e| e.to_string())?;
    info!("Loaded installed engines: {:?}", engines);
    Ok(engines)
}

#[tauri::command]
pub async fn get_available_releases(state: State<'_, AppState>) -> Result<Vec<ogl_core::EngineRelease>, String> {
    let releases = state.launcher_service.list_available_releases().await.map_err(|e| e.to_string())?;
    info!("Loaded available releases: {:?}", releases);
    Ok(releases)
}

#[tauri::command]
pub async fn launch_game(state: State<'_, AppState>, game: GothicGame) -> Result<(), String> {
    info!("Launching game: {:?}", game);
    state.launcher_service.launch_profile(&game.profile_id()).await.map_err(|e| {
        let msg = e.to_string();
        error!("Launch failed for {:?}: {}", game, msg);
        msg
    })
}

#[tauri::command]
pub async fn scan_for_games(app: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let installs = state.launcher_service.scan_for_installations(Arc::new(|_| {})).await.map_err(|e| {
        let msg = e.to_string();
        error!("Game scan failed: {}", msg);
        msg
    })?;
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
    state.launcher_service.save_config(&config).await.map_err(|e| {
        let msg = e.to_string();
        error!("Failed to save manual path for {:?}: {}", game, msg);
        msg
    })?;
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
        .map_err(|e| {
            let msg = e.to_string();
            error!("Install failed for {}: {}", version, msg);
            msg
        })?;
        
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn delete_engine(app: AppHandle, state: State<'_, AppState>, version: String) -> Result<(), String> {
    state.launcher_service.delete_engine(&version).await.map_err(|e| {
        let msg = e.to_string();
        error!("Delete failed for {}: {}", version, msg);
        msg
    })?;
    broadcast_state(&app, &state).await
}

#[tauri::command]
pub async fn reinstall_engine(app: AppHandle, state: State<'_, AppState>, version: String) -> Result<(), String> {
    info!("Reinstalling engine version: {}", version);
    state.launcher_service.delete_engine(&version).await.map_err(|e| e.to_string())?;
    
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
        .map_err(|e| {
            let msg = e.to_string();
            error!("Reinstall failed for {}: {}", version, msg);
            msg
        })?;
        
    broadcast_state(&app, &state).await
}
