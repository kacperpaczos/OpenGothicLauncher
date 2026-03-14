# Local Development Setup

## Prerequisites
1.  **Rust Toolchain**: Latest stable (via `rustup`).
2.  **GTK4 Libraries**:
    - **Linux**: `libgtk-4-dev`, `pkg-config`.
    - **Windows**: `gvsbuild` or MSYS2-based GTK4 distribution.
3.  **Build Tools**: `gcc`, `make`, `bash`.

## Setup Steps
1.  Clone the repository: `git clone <repo-url>`.
2.  Install dependencies: `cargo fetch`.
3.  Run the GUI in dev mode: `./dev/run_gui.sh`.

## Environment Variables
- `RUST_LOG`: Set to `debug` or `trace` for detailed logs.
- `APP_PATHS_MODE`: (Internal dev flag) can be used to switch between storage in home directory vs local project directory for testing purposes.
