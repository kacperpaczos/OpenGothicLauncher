# Runtime Flow

This document describes the temporal behavior of the application during common user interactions.

## 1. Application Startup Flow
1.  **Entry Point**: `main.rs` in `ogl-gui` initializes the `Tokio` runtime.
2.  **DI Container**: Adapters (Infra, Network, Executor) are instantiated.
3.  **Service Initialization**: `LauncherService` is created with dependencies injected as `Arc<dyn Port>`.
4.  **State Loading**: The service calls `ConfigStore::load()` via a port.
5.  **GUI Presentation**: The GTK window is built and mapped to the `SharedUiState`.
6.  **Background Check**: A background task triggers `LauncherService::list_installed_engines()`.

## 2. Engine Installation Runtime
1.  **User Action**: Click "Install" in `EngineWindow`.
2.  **Async Task**: A Tokio task is spawned.
3.  **Network Call**: `ReleaseProvider::latest_release()` fetches metadata from GitHub.
4.  **Streaming**: `EngineDownloader::download()` streams the ZIP while pushing progress updates to the UI channel.
5.  **Extraction**: Once complete, `ArchiveExtractor::extract_zip()` runs on the thread pool.
6.  **Switch**: The `SharedUiState` is updated, triggering a UI refresh.

## 3. Game Launch Flow
1.  **Launch Command**: `LauncherService::launch_profile(game)`.
2.  **Arg Assembly**: The service gathers game paths, engine paths, and mod lists.
3.  **Execution**: `GameProcessRunner::launch()` is called.
4.  **Child Monitoring**: `ogl-executor` spawns the process and returns a handle.
5.  **Watchdog**: The UI remains responsive while the game runs in a separate process tree.
