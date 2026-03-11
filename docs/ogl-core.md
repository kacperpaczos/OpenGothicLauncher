# ogl-core

`ogl-core` is the foundational library of the OpenGothicLauncher. It is designed to be completely independent of any graphical or terminal user interfaces, focusing purely on business logic and state management.

## Key Responsibilities

1. **Installation Detection (`install_detector`)**: Cross-platform detection of Gothic installations using a 3-stage algorithm (see below). Supports all Gothic variants.
2. **Configuration Management (`config_manager`)**: Handles persistent user settings. Reads and writes TOML files to the OS-native config directory (profiles, engine versions, paths).
3. **Engine Management (`engine_manager`)**: Manages downloaded OpenGothic engine versions in the OS data directory and lists which are available to launch.
4. **Sandbox Management (`sandbox_manager`)**: Prepares isolated working directories for different game profiles, allowing separate mod loadouts and save files without touching the base installation.

## Supported Game Variants

The `GothicGame` enum represents every supported title:

| Variant | Game |
|---------|------|
| `Gothic1` | Gothic |
| `Gothic2` | Gothic II (vanilla, no NotR) |
| `Gothic2NotR` | Gothic II: Night of the Raven (Noc Kruka) |
| `ChroniclesOfMyrtana` | The Chronicles of Myrtana: Archolos |
| `Gothic3` | Gothic 3 |

Gothic II vanilla vs. Night of the Raven is distinguished automatically by the presence of `Data/Addon.vdf`.

## Installation Detection — 3-Stage Algorithm

```
detect(GothicGame::Gothic2NotR)
```

### Stage 1 – Fast (~0 ms)
Checks platform-specific registries and well-known paths:
- **Windows**: Steam registry (`HKCU\SOFTWARE\Valve\Steam`), GOG per-game registry keys, hardcoded roots (`C:\Games`, `D:\Games`…)
- **Linux**: `~/.steam/steam/steamapps/common/`, `~/.local/share/Steam/…`, Wine/Lutris prefixes
- **macOS**: `~/Library/Application Support/Steam/steamapps/common/`

### Stage 2 – Heuristic (~2 s)
Shallow scan (depth ≤ 2) of user-writable locations. Filters folder names containing `gothic`, `archolos`, or `myrtana` before validating.

### Stage 3 – Brute Force (opt-in, async)
Full-disk walk via `detect_brute_force(game, on_progress)`. Skips system directories (`/proc`, `Windows\`, etc.). Accepts a progress callback for GUI display.

```rust
// Fast detect (Stage 1 + 2)
let install = detect(GothicGame::Gothic2NotR)?;

// Full disk scan with progress (Stage 3)
let install = detect_brute_force(GothicGame::Gothic3, |path| {
    println!("Scanning: {}", path.display());
})?;
```

## Architecture

Depends only on `dirs` (system paths), `serde` (serialization), `thiserror` (errors), and on Windows builds: `windows-registry` (Registry API). All other crates (`ogl-cli`, `ogl-gui`) pull from `ogl-core` to make decisions.
