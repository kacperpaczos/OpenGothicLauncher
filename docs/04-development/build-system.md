# Build System

The project uses the standard Rust build system, **Cargo**, managed through a workspace.

## Build Configurations
- **Dev Profile**: Default. Optimized for compilation speed with debug symbols.
- **Release Profile**: Enabled via `--release`. Optimized for performance with Link Time Optimization (LTO) set to `thin`.

## Helper Scripts
- **`dev/build.sh`**: Wraps `cargo build`. Supports `--release` flag to aggregate build commands.
- **`dev/run_gui.sh`**: The primary developer entry point. It sets `RUST_LOG=debug`, runs the `ogl-gui` crate, and pipes output to `dev/gui_log.txt` using `tee`.

## Build Artifacts
Binary artifacts are generated in the root `target/` directory:
- `target/debug/ogl-gui` / `target/debug/ogl-cli`
- `target/release/ogl-gui` / `target/release/ogl-cli`
