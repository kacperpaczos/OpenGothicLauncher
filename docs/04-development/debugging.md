# Debugging

## Log-Based Debugging
The project heavily utilizes structured logging via the `tracing` crate.
- Logs are printed to `stdout` and mirrored to `dev/gui_log.txt` when using `run_gui.sh`.
- Key spans: `launcher_service`, `install_detector`, `network_adapter`.

## GDB / LLDB
- Since the project is built in debug mode by default (in dev), standard debuggers like GDB or LLDB can be attached to the `ogl-gui` process.
- VSCode Integration: Use the `CodeLLDB` extension with the provided `.vscode/launch.json` (if available) or by auto-generating a Rust launch configuration.

## GTK Inspector
When running the GUI, you can use the GTK interactive debugger by setting `GTK_DEBUG=interactive` before launching. This allows for real-time inspection of the widget tree and CSS styling.
