use std::sync::OnceLock;
use tokio::runtime::Runtime;

/// Global tokio runtime for background async work (network, detection).
///
/// GTK4 has its own main loop, so we run tokio in a separate thread.
/// GUI code spawns tasks via `background().spawn(...)` and receives
/// results back on the GTK main thread via `glib::MainContext::default().spawn_local(...)`.
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

/// Get (or initialize) the global tokio runtime.
pub fn background() -> &'static Runtime {
    RUNTIME.get_or_init(|| {
        Runtime::new().expect("Failed to create tokio runtime")
    })
}
