# OpenGothicLauncher

A cross-platform launcher and runtime manager for the Gothic series.

## Supported Games

| Game | Variant |
|------|---------|
| Gothic | All editions |
| Gothic II | Vanilla |
| Gothic II: Night of the Raven | Noc Kruka (NK) |
| The Chronicles of Myrtana: Archolos | Total conversion for G2 |
| Gothic 3 | |

## Features

- **Auto-detect** Gothic installations on Windows (Steam + GOG Registry), Linux (Steam, Wine, Lutris), and macOS
- **3-stage path finder**: fast registry lookup → shallow heuristic scan → opt-in full disk scan
- **Download, update, and manage** the OpenGothic engine (from GitHub Releases, SHA-256 verified)
- **Launch Gothic** with any OpenGothic engine version
- **Mod Management**: detect `.vdf` / `.mod` files and manage load order
- **Sandboxes**: handle multiple isolated game configurations / profiles
- **GTK4 GUI** and headless **CLI** interface

## Crate Structure

| Crate | Role |
|-------|------|
| `ogl-core` | Business logic: detection, config, engine & sandbox management |
| `ogl-network` | HTTP downloads from GitHub, SHA-256 integrity checks |
| `ogl-executor` | Async game process launcher with stdout/stderr capture |
| `ogl-mods` | Scan and load-order mod manager |
| `ogl-cli` | Headless CLI (clap) |
| `ogl-gui` | GTK4 graphical interface |

## Quick Start

See [CONTRIBUTING.md](CONTRIBUTING.md) for build instructions.
