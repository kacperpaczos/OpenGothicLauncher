# ogl-cli

`ogl-cli` provides a fully featured, headless, command-line interface for the launcher. It essentially wraps the inner crates (`core`, `network`, `executor`, `mods`) and exposes them as subcommands.

## Key Responsibilities

1. **Argument Parsing**: Uses the `clap` crate with a declarative struct macro format to parse arguments and flags.
2. **Headless Execution**: Ideal for writing bash scripts to auto-update OpenGothic or for running the launcher on servers / CI systems without a display server.
3. **Structured Output**: Prints colored, formatted logs using `tracing` and `tracing-subscriber`.

## Commands

- `ogl-cli detect` - Auto-detects the Gothic installation folder.
- `ogl-cli engines` - Lists currently installed OpenGothic runtimes.
- `ogl-cli mods` - Scans and lists all found modifications in the data directory.
