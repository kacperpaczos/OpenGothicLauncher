# Code Style

## Rust Standards
- **Formatting**: The project adheres strictly to `rustfmt` defaults.
- **Naming**: `snake_case` for functions/variables, `PascalCase` for types/traits, `SCREAMING_SNAKE_CASE` for constants.
- **Linting**: Uses `clippy` for static analysis.

## Conventions
- **Asynchronicity**: All I/O-bound operations must be `async` and utilize the `tokio` runtime.
- **Explicitness**: Prefer clear, descriptive names over short abbreviations (e.g., `LauncherService` instead of `LService`).
- **Documentation**: All public traits and complex services should have doc-comments (`///`).
- **Error Types**: Always return a `Result` for operations that can fail; avoid `unwrap()` in production code.

## File Organization
- One crate per folder in `crates/`.
- `mod.rs` or `lib.rs` used for module aggregation.
- Sub-modules utilized for separation of concerns within a crate (e.g., `src/ports/mod.rs`).
