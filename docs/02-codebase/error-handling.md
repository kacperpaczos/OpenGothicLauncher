# Error Handling

The project uses a structured error-handling strategy based on the `thiserror` and `anyhow` crates.

## 1. Domain-Specific Errors (`CoreError`)
Defined in `ogl-core/src/errors.rs`. These are "clean" errors that describe what went wrong from a business perspective (e.g., `NotFound`, `UnsupportedPlatform`).

## 2. Error Propagation
- **Libraries/Crates**: Use `thiserror` to define precise, enumerable errors. This allows callers (like the GUI) to match on specific error variants and show appropriate user messages.
- **Entry Points (GUI/CLI)**: May use `anyhow` for top-level error reporting where the specific variant is less critical than the backtrace and context.

## 3. Boundary Errors
When an external error occurs (e.g., a `reqwest` error or an `std::io` error), it is caught at the adapter boundary and converted into a `CoreError::External` or `CoreError::Io` before being passed into the Core layer.

## 4. User Feedback
The `ogl-gui` captures errors from background tasks and sets them in the `AppUiState`. The main window then displays these as "Error Banners" to the user, ensuring that background failures don't go unnoticed.
