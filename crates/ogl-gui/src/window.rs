use gtk4::prelude::*;
use gtk4::{
    ApplicationWindow, Box as GtkBox, Orientation, Separator, ScrolledWindow, Image,
};

use crate::view_models::SharedUiState;
use crate::sidebar;
use crate::game_panel::GamePanel;
use crate::view_models::GamePanelViewModel;
use ogl_assets::APP_ICON_PNG;

/// Build the main application window with sidebar + game panel layout.
pub fn build_window(app: &gtk4::Application, state: &SharedUiState, game_panel_vm: GamePanelViewModel) -> ApplicationWindow {
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
        GamePanel::new(state, game_panel_vm.clone())
    ));

    let panel_for_sidebar = game_panel.clone();
    let vm_for_sidebar = game_panel_vm.clone();
    let sidebar_list = sidebar::build_sidebar(&state, move |game| {
        vm_for_sidebar.set_selected_game(game);
        panel_for_sidebar.lock().unwrap().refresh();
    });

    let sidebar_container = GtkBox::new(Orientation::Vertical, 0);
    sidebar_container.set_vexpand(true);

    if let Some(app_icon) = app_icon_widget() {
        sidebar_container.append(&app_icon);
    }

    sidebar_container.append(&sidebar_list);
    sidebar_scroll.set_child(Some(&sidebar_container));
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

fn app_icon_widget() -> Option<Image> {
    let loader = gdk_pixbuf::PixbufLoader::new();
    if loader.write(APP_ICON_PNG).is_ok() && loader.close().is_ok() {
        if let Some(pixbuf) = loader.pixbuf() {
            let image = Image::from_pixbuf(Some(&pixbuf));
            image.set_halign(gtk4::Align::Center);
            image.set_margin_top(12);
            image.set_margin_bottom(12);
            return Some(image);
        }
    }

    None
}
