# Configuration System

The application's persistent state is managed via a centralized configuration system.

## 1. Storage Backend
- **Format**: TOML (chosen for readability and Rust-native support).
- **Location**:
    - **Linux**: `~/.config/opengothiclauncher/state.toml`.
    - **Windows**: `%APPDATA%\OpenGothicLauncher\state.toml`.

## 2. Data Structure (`LauncherConfig`)
Contains:
- `active_engine`: The version tag of the OpenGothic binary to use by default.
- `game_states`: A map of `GothicGame` to `GameState`, storing discovered paths and user preferences for each game variant.

## 3. Persistence Logic
Persistence is handled by the `ConfigStore` port. The `ogl-infra` implementation uses a "Full Rewrite" strategy: whenever the state is modified in memory, the entire `state.toml` is atomically overwritten to ensure consistency and avoid partial corruption.
