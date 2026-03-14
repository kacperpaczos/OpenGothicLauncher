use gtk4::prelude::*;
use gtk4::{Box as GtkBox, Label, Orientation, Button, ListBox, ListBoxRow};

use crate::view_models::SharedUiState;
use crate::view_models::EngineManagerViewModel;

pub fn open_engine_manager_window(
    state: &SharedUiState,
    view_model: &EngineManagerViewModel,
    parent: Option<&gtk4::Window>,
) -> gtk4::Window {
    let window = gtk4::Window::builder()
        .title("OpenGothic Engine Manager")
        .default_width(520)
        .default_height(360)
        .build();

    if let Some(parent) = parent {
        window.set_transient_for(Some(parent));
        if let Some(app) = parent.application() {
            window.set_application(Some(&app));
        }
    }

    let root = GtkBox::new(Orientation::Vertical, 12);
    root.set_margin_top(16);
    root.set_margin_bottom(16);
    root.set_margin_start(16);
    root.set_margin_end(16);

    let header = Label::new(Some("Engine storage"));
    header.set_halign(gtk4::Align::Start);
    header.add_css_class("title-3");
    root.append(&header);

    let engines_dir_label = Label::new(None);
    engines_dir_label.set_halign(gtk4::Align::Start);
    engines_dir_label.set_wrap(true);
    root.append(&engines_dir_label);

    let list_header = Label::new(Some("Available engines"));
    list_header.set_halign(gtk4::Align::Start);
    list_header.add_css_class("title-3");
    list_header.set_margin_top(8);
    root.append(&list_header);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    root.append(&list_box);

    let active_label = Label::new(None);
    active_label.set_halign(gtk4::Align::Start);
    active_label.set_margin_top(8);
    root.append(&active_label);

    let refresh_btn = Button::with_label("Refresh");
    refresh_btn.set_halign(gtk4::Align::Start);
    root.append(&refresh_btn);

    let vm_ref = view_model.clone();
    refresh_btn.connect_clicked(move |_| {
        vm_ref.refresh();
    });

    // Initial populate
    view_model.refresh();
    engines_dir_label.set_text(&view_model.engines_dir_label());

    // Periodic refresh for async updates
    {
        let list_box_ref = list_box.clone();
        let active_label_ref = active_label.clone();
        let state_ref = state.clone();
        glib::timeout_add_local(std::time::Duration::from_millis(500), move || {
            refresh_engine_list(&list_box_ref, &active_label_ref, &state_ref);
            glib::ControlFlow::Continue
        });
    }

    window.set_child(Some(&root));
    window.present();
    window
}

fn refresh_engine_list(list_box: &ListBox, active_label: &Label, state: &SharedUiState) {
    while let Some(child) = list_box.first_child() {
        list_box.remove(&child);
    }

    let s = state.lock().unwrap();
    if s.available_releases.is_empty() {
        let row = ListBoxRow::new();
        row.set_child(Some(&Label::new(Some("No release data available"))));
        list_box.append(&row);
    } else {
        for release in &s.available_releases {
            let installed = s.installed_engines.iter().find(|engine| {
                engine.version == release.tag || engine.version == release.name
            });

            let label_text = if let Some(installed) = installed {
                format!(
                    "{}  •  installed ({})",
                    release.tag,
                    installed.executable_path.display()
                )
            } else {
                format!("{}  •  not installed", release.tag)
            };

            let row = ListBoxRow::new();
            let label = Label::new(Some(&label_text));
            label.set_halign(gtk4::Align::Start);
            label.set_wrap(true);
            row.set_child(Some(&label));
            list_box.append(&row);
        }
    }

    let active = s.config.active_engine.clone().unwrap_or_else(|| "(none)".to_string());
    active_label.set_text(&format!("Active engine: {}", active));
}
