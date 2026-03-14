# Repository Structure

The OpenGothicLauncher repository follows a standard Rust Workspace organization, separating the logical core from infrastructure and multiple frontends.

## Directory Layout
- **`crates/`**: Contains all internal library and binary crates.
    - **`ogl-core/`**: The decoupled business logic (Pure Rust).
    - **`ogl-infra/`**: OS-specific adapters and persistence logic.
    - **`ogl-network/`**: HTTP communication and download management.
    - **`ogl-executor/`**: Process management and game launching.
    - **`ogl-gui/`**: GTK4 Graphical User Interface.
    - **`ogl-cli/`**: Command Line Interface.
    - **`ogl-mods/`**: Archive (VDF) scanning utilities.
    - **`ogl-assets/`**: Static binary resources (icons, images).
- **`docs/`**: Technical documentation (Refined into a numbered structure).
- **`dev/`**: Helper scripts for building and running in dev mode.
- **`Cargo.toml`**: Root workspace configuration and shared dependency definitions.

## Key Files
- `Cargo.lock`: Frozen dependency tree for reproducible builds.
- `dev/run_gui.sh`: Shell script to quickly compile and start the GUI with debug logging.
- `.gitignore`: Standard Rust ignore patterns plus workspace-specific exclusions.
