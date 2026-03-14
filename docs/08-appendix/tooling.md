# Tooling

The following tools are essential for the development and maintenance of OpenGothicLauncher.

## Core Rust Tools
- **rustup**: Managing Rust version and toolchains.
- **cargo**: Build system, runner, and package manager.
- **clippy**: Linter for identifying common bugs and styling issues.
- **rustfmt**: Auto-formatter to ensure a consistent coding style.

## GUI Development Tools
- **GTK Inspector**: Interactive debugger for GTK applications.
- **Cambalache / Glade**: UI designers (though current UI is built procedurally).

## Analysis & Performance
- **cargo-bloat**: Finding what takes up most space in the binary.
- **valgrind / heaptrack**: Memory profiling (particularly on Linux).
- **cargo-expand**: Inspecting code after macro expansion.

## Documentation
- **mdBook**: (Optional) could be used to render these markdown files into a searchable web interface.
- **Mermaid.js**: Used for generating architecture diagrams within these markdown documents.
