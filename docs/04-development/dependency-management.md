# Dependency Management

## Cargo Workspace
Dependencies are centrally managed in the root `Cargo.toml` under `[workspace.dependencies]`. Internal crates reference these using `anyhow.workspace = true`, ensuring version consistency across the entire system.

## Key External Dependencies
- **Asynchronous Runtime**: `tokio` (full features).
- **Networking**: `reqwest` (with JSON and Rustls support).
- **Serialization**: `serde` and `serde_json` / `toml`.
- **UI Framework**: `gtk4` and `glib`.
- **Error Handling**: `thiserror` for precise library errors and `anyhow` for app-level context.
- **Logging**: `tracing` for structured diagnostic information.

## Update Policy
- Semantic Versioning (SemVer) is followed.
- Dependencies are locked via `Cargo.lock` to ensure build reproducibility across different developer machines and CI environments.
