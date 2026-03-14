# Project Overview

OpenGothicLauncher is a specialized management utility and launcher for the **OpenGothic** engine (a modern re-implementation of the ZenGin engine). The project is designed as a modular Rust workspace, prioritizing platform isolation, ease of engine maintenance, and a seamless user experience for Gothic series enthusiasts.

## Technical Identity
- **Language**: Rust (Edition 2021)
- **Architecture**: Clean Architecture / Hexagonal (Ports & Adapters)
- **UI Framework**: GTK4 (via `ogl-gui`) and Clap (via `ogl-cli`)
- **Runtime**: Tokio (Asynchronous I/O)

## Core Value Proposition
The launcher bridges the gap between legacy game assets and modern engine binaries by automating the detection of original game files, managing engine releases from GitHub, and providing an isolated execution environment.
