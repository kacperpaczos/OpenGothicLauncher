# ogl-gui

`ogl-gui` is the graphical front-end for the launcher, providing a user-friendly interface to manage installations and preferences.

## Key Responsibilities

1. **Window Management**: Written using `gtk4-rs`, providing a native look-and-feel on Linux desktops (GNOME/KDE) and functioning smoothly across Windows and macOS setups using the GTK runtime.
2. **State Binding**: Retrieves state data (like detected paths) from `ogl-core` and visualizes it immediately.
3. **User Events**: Connects buttons (like "Launch" and "Manage Engines") to the asynchronous logic found in `ogl-executor` and `ogl-network`.

## Architecture

GTK4 operates on an event-driven main loop. Heavy workloads (like downloading engines via `ogl-network` or waiting for the `ogl-executor` process) must be sent to separate `tokio` threadpools to avoid freezing the graphical interface. 

Currently implemented as a scaffold app with foundational views.
