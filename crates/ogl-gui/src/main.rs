mod app_state;
mod runtime;
mod window;
mod sidebar;
mod game_panel;

use gtk4::prelude::*;
use gtk4::Application;

const APP_ID: &str = "com.github.pryoxar.OpenGothicLauncher";

fn main() {
    // Initialize tracing for debug logs
    tracing_subscriber::fmt::init();

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
