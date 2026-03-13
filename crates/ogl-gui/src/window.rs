use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box as GtkBox, Orientation, Separator, ScrolledWindow,
};

use crate::app_state::SharedState;
use crate::sidebar;
use crate::game_panel::GamePanel;

/// Build the main application window with sidebar + game panel layout.
pub fn build_window(app: &gtk4::Application, state: &SharedState) -> ApplicationWindow {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("OpenGothic Launcher")
        .default_width(780)
        .default_height(480)
        .build();

    let root_box = GtkBox::new(Orientation::Horizontal, 0);

    // ── Left: sidebar ──
    let sidebar_scroll = ScrolledWindow::new();
    sidebar_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    sidebar_scroll.set_width_request(200);

    let game_panel = std::sync::Arc::new(std::sync::Mutex::new(
        GamePanel::new(state)
    ));

    let state_for_sidebar = state.clone();
    let panel_for_sidebar = game_panel.clone();
    let sidebar_list = sidebar::build_sidebar(&state, move |game| {
        state_for_sidebar.lock().unwrap().selected_game = game;
        state_for_sidebar.lock().unwrap().error_message = None;
        panel_for_sidebar.lock().unwrap().refresh();
    });

    sidebar_scroll.set_child(Some(&sidebar_list));
    root_box.append(&sidebar_scroll);

    // Vertical separator between sidebar and panel
    let sep = Separator::new(Orientation::Vertical);
    root_box.append(&sep);

    // ── Right: game panel ──
    let panel_scroll = ScrolledWindow::new();
    panel_scroll.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
    panel_scroll.set_hexpand(true);
    panel_scroll.set_vexpand(true);

    let panel_container = &game_panel.lock().unwrap().container;
    panel_scroll.set_child(Some(panel_container));
    root_box.append(&panel_scroll);

    window.set_child(Some(&root_box));

    // ── Periodic refresh for async operations (download progress, detection) ──
    let panel_for_tick = game_panel.clone();
    glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
        panel_for_tick.lock().unwrap().refresh();
        glib::ControlFlow::Continue
    });

    window
}
