# OpenGothicLauncher - Developer Guide

## Prerequisites (Linux)
To compile the GTK4 graphical interface (`ogl-gui`), you will need the GTK4 development libraries:

```bash
sudo apt install libgtk-4-dev build-essential
```

## Building the Project
To compile the entire workspace (all libraries, CLI, and GUI):

```bash
cargo build
```

Or, to build a release version:

```bash
cargo build --release
```

## Running the GUI
To start the GTK4 graphical interface directly from source:

```bash
cargo run -p ogl-gui
```

## Running the CLI
To interact with the headless command-line tools:

```bash
cargo run -p ogl-cli -- help
```
