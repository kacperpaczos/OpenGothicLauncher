# ogl-mods

`ogl-mods` provides file-system-level inspection of the original Gothic `Data/` and `mods/` directories to figure out what game modifications are available to the user.

## Key Responsibilities

1. **Modification Scanning**: Reads directories recursively to find `.vdf` (Virtual Disk File) and `.mod` archives used by the ZenGin engine/OpenGothic.
2. **Metadata Parsing**: Extracts the conceptual name of the mods based on filenames (and in the future, by parsing `.ini` configuration blocks inside the mods).
3. **Load Order Resolving**: Takes a user's selected list of mods from their active profile and structures a deterministic load-order array to feed to the `ogl-executor`.

## Architecture

Synchronous file reading. Designed to be fast and safe. If permissions deny access to the Data folder, it fails gracefully returning an empty list rather than crashing.
