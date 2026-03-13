mod app_state;
mod runtime;
mod window;
mod sidebar;
mod game_panel;
mod engine_window;

use gtk4::prelude::*;
use gtk4::Application;


const APP_ID: &str = "com.github.paczos.OpenGothicLauncher";

fn main() {
    // Initialize tracing for debug logs to both stdout and a file
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("OpenGothicLauncher")
        .join("logs");
        
    std::fs::create_dir_all(&config_dir).unwrap_or_default();
    
    let file_appender = tracing_appender::rolling::daily(config_dir, "ogl-gui.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(
            // Write to stdout and the rotating file
            tracing_subscriber::fmt::writer::MakeWriterExt::and(std::io::stdout, non_blocking)
        )
        .init();
        
    tracing::info!("Starting OpenGothicLauncher GUI...");

    let app = Application::builder()
        .application_id(APP_ID)
        .build();

    app.connect_activate(|app| {
        // Load persisted state from disk
        let state = app_state::new_shared_state();
        
        // Build and present the main window
        let win = window::build_window(app, &state);
        win.present();
    });

    app.run();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_gui_compiles_and_harness_works() {
        // GTK startup tests require a display server (X11/Wayland). 
        // This test ensures that the module compiles correctly and cargo test works.
        assert_eq!(2 + 2, 4);
    }
}
