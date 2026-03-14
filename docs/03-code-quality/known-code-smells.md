# Known Code Smells

As the project is undergoing a re-architecture, several "smells" are present and scheduled for refactoring.

1.  **Legacy Core Bloat**: `ogl-core/src/install_detector.rs` contains concrete detection logic. In a pure Clean Architecture, this should only be in `ogl-infra`.
2.  **Shared State Mutex**: `SharedUiState` uses a single large Mutex. This can lead to contention as more features are added.
3.  **Redundant Mapping**: Some domain objects in `ogl-core` are very similar to persistence structs in `ogl-infra`. While good for isolation, it creates boilerplate.
4.  **Implicit Initialization**: Reliance on the user manually running `detect` if the config is missing, rather than a more robust first-start wizard.
5.  **GTK-GDK Pixbuf Loading**: The icon loading logic in `window.rs` is a bit low-level and could be abstracted into a dedicated asset manager service.
