# Internal APIs

This document describes the primary programmatic interfaces within the workspace.

## 1. `LauncherService` (The Facade)
Located in `ogl-core/src/services/launcher_service.rs`.
- `new(dependencies...)`: Constructor for injecting adapters.
- `install_open_gothic(version)`: Asynchronous workflow for engine setup.
- `launch_profile(game)`: High-level orchestration for starting the engine.
- `scan_for_installations()`: Triggers background discovery.

## 2. Port Definitions
Traits in `ogl-core/src/ports/` serve as the "Internal API" between the core logic and the adapters. Implementing a new platform version involves fulfilling these contracts.

## 3. `AppUiState` (GUI Internal)
Located in `ogl-gui/src/view_models/mod.rs`.
- Serves as the communication bus between the background services and the GTK widgets. It is designed for observability, allowing the GUI to react to changes in detection results or download percentages.
