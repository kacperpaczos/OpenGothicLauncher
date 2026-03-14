# Module Dependency Graph

The following graph illustrates the compile-time dependencies between workspace crates. Notice the central position of `ogl-core`, which has no outgoing dependencies within the workspace, adhering to the Clean Architecture rule.

```mermaid
graph TD
    %% Frontends
    GUI[ogl-gui]
    CLI[ogl-cli]

    %% Core
    Core[ogl-core]

    %% Infrastructure & Adapters
    Infra[ogl-infra]
    Net[ogl-network]
    Exec[ogl-executor]
    Mods[ogl-mods]
    Assets[ogl-assets]

    %% Dependencies
    GUI --> Core
    GUI --> Net
    GUI --> Infra
    GUI --> Exec
    GUI --> Assets

    CLI --> Core
    CLI --> Net
    CLI --> Infra
    CLI --> Exec

    Infra --> Core
    Infra --> Net
    Infra --> Assets
    
    Net --> Core
    Exec --> Core
    Mods --> Core

    %% External
    Core -.-> Std[Rust std]
    Net -.-> Reqwest[reqwest / tokio]
    Infra -.-> Serde[serde / toml]
    GUI -.-> GTK[gtk4 / glib]
```

### Dependency Rules
- **Rule 1**: No module can depend on `ogl-gui` or `ogl-cli`.
- **Rule 2**: `ogl-core` must remain independent of all other internal crates.
- **Rule 3**: Circular dependencies are strictly prohibited by the Rust compiler and the architectural design.
