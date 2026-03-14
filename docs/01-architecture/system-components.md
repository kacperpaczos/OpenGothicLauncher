# System Components

The project is composed of several specialized crates, each with a clear responsibility within the hexagonal framework.

## 1. Core Component (`ogl-core`)
The central engine of the application.
- **Sub-modules**:
    - `domain`: `GameState`, `EngineVersion`, `GothicInstall`.
    - `ports`: `ConfigStore`, `InstallDetector`, `ReleaseProvider`, `FileSystem`.
    - `services`: `LauncherService`.

## 2. Infrastructure Adapter (`ogl-infra`)
Direct implementation of ports requiring OS interaction.
- **Responsibility**: Filesystem I/O, config persistence (TOML), archive extraction (ZIP), and Windows Registry access.

## 3. Network Adapter (`ogl-network`)
Adapter for all HTTP-related operations.
- **Responsibility**: Communicating with GitHub API, streaming downloads with progress reporting, and SHA256 integrity checks.

## 4. Execution Adapter (`ogl-executor`)
Abstraction over child process management.
- **Responsibility**: Launching the OpenGothic binary with correctly injected `-g` and `-game` arguments.

## 5. Frontends (`ogl-gui` & `ogl-cli`)
User interaction layers.
- **`ogl-gui`**: GTK4-based visual interface using an MVVM pattern.
- **`ogl-cli`**: Terminal interface using `clap` for automation and headless use.

## 6. Utilities (`ogl-assets` & `ogl-mods`)
- **`ogl-assets`**: Bundled icons and static resources.
- **`ogl-mods`**: VDF/MOD scanning logic and load order resolution.
