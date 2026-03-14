# Core Abstractions

This document highlights the fundamental abstractions that drive the system's extensibility.

## 1. The "Port" Trait Pattern
Most external dependencies are behind an `async-trait`.
```rust
#[async_trait]
pub trait ConfigStore: Send + Sync {
    async fn load(&self) -> Result<LauncherConfig, CoreError>;
    async fn save(&self, config: &LauncherConfig) -> Result<(), CoreError>;
}
```
This allows `LauncherService` to operate without knowing whether it's talking to a local file, a database, or a mock for testing.

## 2. Shared UI State (`AppUiState`)
Used in `ogl-gui` to provide a single, thread-safe source of truth for the UI thread and background Tokio tasks.
- **Concurrency**: Wrapped in `Arc<Mutex<AppUiState>>`.
- **Reactivity**: The UI periodically polls this state (every 500ms) to update progress bars and labels.

## 3. GothicGame Enumeration
The central discriminator for all game-specific logic.
```rust
pub enum GothicGame {
    Gothic1,
    Gothic2,
    Gothic2NotR,
    ChroniclesOfMyrtana,
    Gothic3,
}
```
It is used in discovery, engine versioning, and mod scanning to determine file naming conventions and validation rules.

## 4. GameLaunch Context
A unified object that contains every piece of data needed to start a process: paths, active engine, and resolved mod parameters. It decouples the *preparation* of the launch from the *execution*.
