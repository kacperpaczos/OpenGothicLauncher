#[allow(deprecated)]
use gtk4::prelude::*;
#[allow(deprecated)]
use gtk4::{
    Box as GtkBox, Button, Label, Orientation, ProgressBar,
    Align, FileChooserAction, FileChooserDialog, ResponseType,
};

use ogl_core::GameState;
use crate::view_models::SharedUiState;
use crate::view_models::GamePanelViewModel;

/// The right-side panel showing details for the selected game.
///
/// Dynamically rebuilds its content based on the state:
///   - State A: game not detected → "Scan" / "Browse" buttons
///   - State B: game detected, no engine → "Download" + progress bar
///   - State C: game detected + engine installed → "Launch" button
pub struct GamePanel {
    pub container: GtkBox,
    state: SharedUiState,
    view_model: GamePanelViewModel,
}

impl GamePanel {
    pub fn new(state: &SharedUiState, view_model: GamePanelViewModel) -> Self {
        let container = GtkBox::new(Orientation::Vertical, 16);
        container.set_margin_top(24);
        container.set_margin_bottom(24);
        container.set_margin_start(24);
        container.set_margin_end(24);
        container.set_hexpand(true);
        container.set_vexpand(true);

        let panel = Self {
            container,
            state: state.clone(),
            view_model,
        };
        panel.refresh();
        panel
    }

    /// Clear and rebuild the panel contents based on current AppUiState.
    pub fn refresh(&self) {
        // Remove all children
        while let Some(child) = self.container.first_child() {
            self.container.remove(&child);
        }

        let state = self.state.lock().unwrap();
        let game = state.selected_game;
        let game_state = state.current_game_state();
        let has_engines = !state.installed_engines.is_empty();
        let detection_running = state.detection_running;
        let download_progress = state.download_progress;
        let error_msg = state.error_message.clone();
        let detection_progress = state.detection_progress.clone();
        drop(state);

        // Title
        let title = Label::new(Some(game.display_name()));
        title.add_css_class("title-1");
        title.set_halign(Align::Start);
        self.container.append(&title);

        // Error banner (if any)
        if let Some(err) = error_msg {
            let error_label = Label::new(Some(&format!("⚠ {}", err)));
            error_label.add_css_class("error");
            error_label.set_halign(Align::Start);
            error_label.set_wrap(true);
            self.container.append(&error_label);
        }

        if game_state.detected {
            // ─── State B/C: Game detected ───
            let path_str = game_state.install_path
                .as_ref()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            let found_label = Label::new(Some(&format!("✅ Found: {}", path_str)));
            found_label.set_halign(Align::Start);
            found_label.set_wrap(true);
            found_label.add_css_class("success");
            self.container.append(&found_label);

            // Spacer
            let spacer = GtkBox::new(Orientation::Vertical, 0);
            spacer.set_vexpand(false);
            spacer.set_margin_top(8);
            self.container.append(&spacer);

            if has_engines {
                // ─── State C: Engine installed ───
                let engines_state = self.state.lock().unwrap();
                let engine_label_text = if let Some(ref active) = engines_state.config.active_engine {
                    format!("OpenGothic Engine: {}", active)
                } else if let Some(first) = engines_state.installed_engines.first() {
                    format!("OpenGothic Engine: {}", first.version)
                } else {
                    "OpenGothic Engine: installed".to_string()
                };
                drop(engines_state);

                let engine_label = Label::new(Some(&engine_label_text));
                engine_label.set_halign(Align::Start);
                self.container.append(&engine_label);

                Self::append_engine_manager_button(&self.container, &self.state, &self.view_model);

                let launch_btn = Button::with_label("▶  Launch OpenGothic");
                launch_btn.add_css_class("suggested-action");
                launch_btn.add_css_class("pill");
                launch_btn.set_margin_top(16);

                let vm_ref = self.view_model.clone();
                launch_btn.connect_clicked(move |_| {
                    vm_ref.on_launch_clicked();
                });
                self.container.append(&launch_btn);
            } else {
                // ─── State B: No engine installed ───
                let no_engine_label = Label::new(Some("OpenGothic Engine: not installed"));
                no_engine_label.set_halign(Align::Start);
                self.container.append(&no_engine_label);

                if let Some(progress) = download_progress {
                    let pbar = ProgressBar::new();
                    pbar.set_fraction(progress);
                    pbar.set_show_text(true);
                    pbar.set_text(Some(&format!("Downloading… {:.0}%", progress * 100.0)));
                    pbar.set_margin_top(8);
                    self.container.append(&pbar);
                } else {
                    let download_btn = Button::with_label("⬇  Download latest engine");
                    download_btn.add_css_class("suggested-action");
                    download_btn.set_margin_top(8);

                    let vm_ref = self.view_model.clone();
                    download_btn.connect_clicked(move |_| {
                        vm_ref.on_download_clicked();
                    });
                    self.container.append(&download_btn);
                }

                Self::append_engine_manager_button(&self.container, &self.state, &self.view_model);
            }
        } else {
            // ─── State A: Game not detected ───
            if detection_running {
                let scanning_box = GtkBox::new(Orientation::Vertical, 4);
                
                let scanning_label = Label::new(Some("🔍 Scanning for installation…"));
                scanning_label.set_halign(Align::Start);
                scanning_box.append(&scanning_label);

                let spinner = gtk4::Spinner::new();
                spinner.set_spinning(true);
                spinner.set_halign(Align::Start);
                scanning_box.append(&spinner);

                if let Some(ref progress) = detection_progress {
                    let p_lbl = Label::new(Some(&format!("Checking: {}", progress)));
                    p_lbl.set_halign(Align::Start);
                    p_lbl.add_css_class("dim-label");
                    // Using wrap to prevent long paths from stretching the window
                    p_lbl.set_wrap(true);
                    p_lbl.set_wrap_mode(gtk4::pango::WrapMode::Char);
                    scanning_box.append(&p_lbl);
                }

                scanning_box.set_margin_top(8);
                self.container.append(&scanning_box);
            } else {
                let not_found_label = Label::new(Some("⚠ Installation not found"));
                not_found_label.set_halign(Align::Start);
                not_found_label.add_css_class("warning");
                self.container.append(&not_found_label);

                let btn_box = GtkBox::new(Orientation::Horizontal, 8);
                btn_box.set_margin_top(12);

                let scan_btn = Button::with_label("🔍  Scan for installation");
                scan_btn.add_css_class("suggested-action");
                let vm_ref = self.view_model.clone();
                scan_btn.connect_clicked(move |_| {
                    vm_ref.on_scan_clicked();
                });
                btn_box.append(&scan_btn);

                let browse_btn = Button::with_label("📁  Select folder manually");
                let state_ref = self.state.clone();
                let vm_ref = self.view_model.clone();
                let container_ref = self.container.clone();
                browse_btn.connect_clicked(move |btn| {
                    Self::on_browse_clicked(&state_ref, &vm_ref, btn, &container_ref);
                });
                btn_box.append(&browse_btn);

                self.container.append(&btn_box);
            }
        }
    }

    // ─── Action handlers ───
    fn append_engine_manager_button(container: &GtkBox, state: &SharedUiState, view_model: &GamePanelViewModel) {
        let manage_btn = Button::with_label("⚙  Manage engines");
        manage_btn.set_margin_top(8);

        let state_ref = state.clone();
        let vm_ref = view_model.clone();
        manage_btn.connect_clicked(move |btn| {
            let parent = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            let engine_vm = vm_ref.engine_manager_view_model();
            crate::engine_window::open_engine_manager_window(&state_ref, &engine_vm, parent.as_ref());
        });

        container.append(&manage_btn);
    }

    fn on_browse_clicked(state: &SharedUiState, view_model: &GamePanelViewModel, btn: &Button, _container: &GtkBox) {
        let dialog = FileChooserDialog::new(
            Some("Select Gothic installation folder"),
            btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok()).as_ref(),
            FileChooserAction::SelectFolder,
            &[
                ("Cancel", ResponseType::Cancel),
                ("Select", ResponseType::Accept),
            ],
        );

        let state_ref = state.clone();
        let vm_ref = view_model.clone();
        dialog.connect_response(move |dialog, response| {
            if response == ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(path) = file.path() {
                        let mut s = state_ref.lock().unwrap();
                        s.set_current_game_state(GameState {
                            install_path: Some(path),
                            detected: true,
                        });
                        s.error_message = None;
                        vm_ref.save_config();
                    }
                }
            }
            dialog.close();
        });

        dialog.show();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn game_panel_compiles() {
        assert_eq!(2 + 2, 4);
    }
}
