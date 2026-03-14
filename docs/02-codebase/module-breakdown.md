# Module Breakdown

## Internal Crates Analysis

### 1. `ogl-core`
- **Scope**: Entirely platform-agnostic.
- **Key Modules**:
    - `domain/`: Defines the "Language" of the app (Engines, Installs, Configs).
    - `ports/`: Abstract requirements defined as async traits.
    - `services/`: The high-level orchestration layer.

### 2. `ogl-infra`
- **Scope**: Platform-heavy (Registry, Filesystem).
- **Key Modules**:
    - `install_detector`: Implementation of the 3-stage discovery algorithm.
    - `config_store`: TOML-based persistence for the application state.
    - `platform`: Runtime OS detection.

### 3. `ogl-network`
- **Scope**: Networking side-effects.
- **Key Modules**:
    - `releases`: GitHub API integration.
    - `downloads`: Multi-threaded streaming downloads with progress reporting.

### 4. `ogl-gui`
- **Scope**: Presentation.
- **Key Modules**:
    - `view_models`: Bridges the gap between GTK widgets and Core services using shared state.
    - `window/sidebar/game_panel`: Components of the main application window.

### 5. `ogl-mods` & `ogl-executor`
- **`ogl-mods`**: Lightweight utility for reading Gothic-specific archive files.
- **`ogl-executor`**: Low-level thin wrapper around `tokio::process::Command`.
