# State Management

## 1. Core State vs UI State
We maintain a strict separation between logical state and presentational state.

### Core State (`LauncherConfig`)
- **Authority**: `LauncherService`.
- **Persistence**: Managed by `ConfigStore`.
- **Nature**: Persistent data (paths, settings).

### UI State (`AppUiState`)
- **Authority**: `view_models`.
- **Persistence**: Ephemeral (only in memory during execution).
- **Nature**: Transient data (download progress, search status, currently selected game in the sidebar).

## 2. Shared State Synchronization
The `SharedUiState` (Arc+Mutex) is the bridge.
- **Writers**: Background tasks (reporting progress), Service methods (updating discovered paths).
- **Readers**: UI callbacks, polling timers.

## 3. Concurrency Safety
- Mutexes are held for the minimum time possible.
- Complex state transitions are handled via methods on the state struct itself to ensure atomic updates and maintain invariants.
