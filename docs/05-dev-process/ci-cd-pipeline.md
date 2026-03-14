# CI/CD Pipeline

Currently, the project is configured for GitHub Actions (conceptually).

## Pipeline Stages
- **Check**: `cargo check` on multiple targets (Linux/Windows).
- **Lint**: `cargo clippy --all-targets -- -D warnings`.
- **Test**: `cargo test --workspace`.
- **Format**: `cargo fmt --all -- --check`.
- **Build Artifacts**: Parallel jobs for building `.zip` archives of the `ogl-gui` and `ogl-cli` binaries.

## Planned Improvements
- Automated installer creation (e.g., using InnoSetup for Windows or AppImage for Linux).
- CD integration to auto-upload generated binaries to GitHub Releases when a version tag is pushed.
