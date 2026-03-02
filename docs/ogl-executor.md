# ogl-executor

`ogl-executor` is an abstraction layer over the operating system's process management APIs. It is responsible for starting and monitoring the OpenGothic game instance.

## Key Responsibilities

1. **Process Launching**: Assembles the exact command-line arguments needed to start OpenGothic. This includes passing the original Gothic data path (`--game`) and injecting selected mod profiles (`--mod`).
2. **Environment Wrapping**: Executes the binary using `tokio::process::Command`, ensuring it runs non-blocking.
3. **Logging & Monitoring**: Captures the standard output (`stdout`) and error output (`stderr`) of the OpenGothic instance to facilitate debugging. It monitors the exit code of the game and bubbles up panic/crash states to the launcher.

## Architecture

Very thin crate focused exclusively on process wrapping. Relies on `tokio::process` for async child process management.
