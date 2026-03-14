# Terminology

| Term | Technical Description |
| :--- | :--- |
| **OpenGothic** | A modern, open-source re-implementation of the Gothic engine. |
| **ZenGin** | The original proprietary engine used by Piranha Bytes for Gothic 1 & 2. |
| **Workspace** | The Rust root project containing multiple internal crates. |
| **Crate** | A modular unit of Rust code (e.g., `ogl-core`, `ogl-infra`). |
| **Port** | An interface (Rust trait) defined in the Core layer to be implemented by Infrastructure. |
| **Adapter** | A concrete implementation of a Port (e.g., networking via `reqwest`). |
| **Domain** | The innermost layer of the architecture containing pure business logic and models. |
| **VDF** | Virtual Disk File; the standard archive format used by Gothic games. |
| **Sandbox** | An isolated directory structure created by the launcher to separate saves and configs from the original game. |
| **GothicGame** | An enumeration in `ogl-core` representing supported variants (G1, G2, etc.). |
