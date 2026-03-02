# ogl-core

`ogl-core` is the foundational library of the OpenGothicLauncher. It is designed to be completely independent of any graphical or terminal user interfaces, focusing purely on business logic and state management.

## Key Responsibilities

1. **Installation Detection (`install_detector`)**: Scans typical local filesystem paths and system registries (e.g., Windows Registry, Steam directories) to locate an existing installation of Gothic or Gothic II.
2. **Configuration Management (`config_manager`)**: Handles persistent user settings. It reads and writes TOML files to the OS's native configuration directory, storing profiles, engine versions, and paths.
3. **Engine Management (`engine_manager`)**: Manages local engine installations in the OS data directory. Evaluates which versions of OpenGothic have been downloaded and are available to launch.
4. **Sandbox Management (`sandbox_manager`)**: Prepares isolated environments for different game profiles. This allows players to have separate mod loadouts and save files without polluting the main Gothic installation.

## Architecture

This crate only relies on standard abstractions (`dirs` for system paths, `serde` for serialization, `thiserror` for error handling). Other crates pull data from `ogl-core` to make decisions.
