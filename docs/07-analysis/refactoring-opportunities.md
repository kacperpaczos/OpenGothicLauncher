# Refactoring Opportunities

## 1. Full Decoupling of `install_detector`
Move the remaining logic in `ogl-core/src/install_detector.rs` to `ogl-infra`. Rewrite it to purely implement the `InstallDetector` port.
- **Priority**: High.
- **Status**: Partially completed in the current branch.

## 2. GUI Component Modularization
The `game_panel.rs` is currently a large file with many states. Refactor it into smaller, specialized widgets (e.g., `ScanWidget`, `DownloadWidget`, `LaunchWidget`).
- **Priority**: Medium.

## 3. Error Mapping Standardization
Implement a more robust `From` conversion system for error propagation between adapters and core to reduce boilerplate mapping.
- **Priority**: Low.

## 4. View Model Abstraction
Introduce a generic `ViewModel` trait to standardize how GTK widgets interact with the shared state.
- **Priority**: Medium.
