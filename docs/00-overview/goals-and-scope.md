# Goals and Scope

## Goals
1.  **Automation of Engine Lifecycle**: Automate the discovery, download, extraction, and updates of OpenGothic engine binaries.
2.  **Zero-Impact Installation**: Ensure that the launcher and OpenGothic do not modify the original Gothic game files, preserving the integrity of the base installation.
3.  **Cross-Platform Parity**: Provide a consistent experience across Linux and Windows by abstracting platform-specific concerns (registry, file paths).
4.  **Extensibility**: Allow for easy addition of new features like mod management and advanced sandboxing through a modular architecture.

## Scope
### In-Scope
- Automatic detection of Gothic 1, 2, 2:NotR, and Chronicles of Myrtana: Archolos.
- Integrated download manager for OpenGothic releases.
- Sandbox profile management (saves, configs).
- GUI (GTK4) and CLI interfaces.
- VDF/MOD file scanning and load order management.

### Out-of-Scope
- Direct modification of Gothic engine source code (OpenGothic is an external dependency).
- Asset extraction or de-compilation of original `.vdf` files.
- Built-in mod downloader (currently limited to local scanning).
