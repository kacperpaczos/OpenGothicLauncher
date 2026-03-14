# Testing Strategy

## Unit Testing
- **Location**: Inline `mod tests` within the same file as the code.
- **Scope**: Testing domain logic and pure functions (e.g., VDF version parsing, SHA256 string formatting).
- **Execution**: `cargo test`.

## Integration Testing
- **Infrastructure Mocking**: Using Ports and Adapters, we can create mocks for the `FileSystem` and `Network` ports to test the `LauncherService` orchestration without side effects.
- **External Mocks**: The `mockito` crate is used in `ogl-network` to test HTTP response handling and failure scenarios.

## UI Testing
- Currently, UI testing is performed manually. Future work includes implementing automated GTK widget tests using `libadwaita` or GTK's native testing framework.

## CI Verification
Every PR is (conceptually) run through:
1. `cargo check` (Type safety).
2. `cargo test` (Unit/Integration tests).
3. `cargo clippy` (Linting standards).
