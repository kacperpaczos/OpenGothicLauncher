#[allow(deprecated)]
use gtk4::prelude::*;
#[allow(deprecated)]
use gtk4::{
    Box as GtkBox, Button, Label, Orientation, ProgressBar,
    Align, FileChooserAction, FileChooserDialog, ResponseType,
};

use ogl_core::config_manager::GameState;
use crate::app_state::SharedState;
use crate::runtime;

/// The right-side panel showing details for the selected game.
///
/// Dynamically rebuilds its content based on the state:
///   - State A: game not detected → "Scan" / "Browse" buttons
///   - State B: game detected, no engine → "Download" + progress bar
///   - State C: game detected + engine installed → "Launch" button
pub struct GamePanel {
    pub container: GtkBox,
    state: SharedState,
}

impl GamePanel {
    pub fn new(state: &SharedState) -> Self {
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
        };
        panel.refresh();
        panel
    }

    /// Clear and rebuild the panel contents based on current AppState.
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

                Self::append_engine_manager_button(&self.container, &self.state);

                let launch_btn = Button::with_label("▶  Launch OpenGothic");
                launch_btn.add_css_class("suggested-action");
                launch_btn.add_css_class("pill");
                launch_btn.set_margin_top(16);

                let state_ref = self.state.clone();
                launch_btn.connect_clicked(move |_| {
                    Self::on_launch_clicked(&state_ref);
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

                    let state_ref = self.state.clone();
                    download_btn.connect_clicked(move |_| {
                        Self::on_download_clicked(&state_ref);
                    });
                    self.container.append(&download_btn);
                }

                Self::append_engine_manager_button(&self.container, &self.state);
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
                let state_ref = self.state.clone();
                scan_btn.connect_clicked(move |_| {
                    Self::on_scan_clicked(&state_ref);
                });
                btn_box.append(&scan_btn);

                let browse_btn = Button::with_label("📁  Select folder manually");
                let state_ref = self.state.clone();
                let container_ref = self.container.clone();
                browse_btn.connect_clicked(move |btn| {
                    Self::on_browse_clicked(&state_ref, btn, &container_ref);
                });
                btn_box.append(&browse_btn);

                self.container.append(&btn_box);
            }
        }
    }

    // ─── Action handlers ───
    fn append_engine_manager_button(container: &GtkBox, state: &SharedState) {
        let manage_btn = Button::with_label("⚙  Manage engines");
        manage_btn.set_margin_top(8);

        let state_ref = state.clone();
        manage_btn.connect_clicked(move |btn| {
            let parent = btn.root().and_then(|r| r.downcast::<gtk4::Window>().ok());
            crate::engine_window::open_engine_manager_window(&state_ref, parent.as_ref());
        });

        container.append(&manage_btn);
    }

    fn on_scan_clicked(state: &SharedState) {
        let game = { state.lock().unwrap().selected_game };
        { 
            let mut s = state.lock().unwrap();
            s.detection_running = true; 
            s.detection_progress = None;
        }

        let state_weak = std::sync::Arc::downgrade(state);
        let progress_weak = state_weak.clone();

        // Run detection on the tokio runtime, send result back to GTK main thread
        runtime::background().spawn(async move {
            use std::sync::atomic::{AtomicI64, Ordering};
            use std::sync::Arc;
            
            // Limit UI updates to every ~50ms to avoid flooding GTK event loop
            let last_update = Arc::new(AtomicI64::new(0));

            let result = ogl_core::detect(game, move |path| {
                tracing::debug!("Scanning path: {}", path.display());
                
                let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64;
                if now - last_update.load(Ordering::Relaxed) > 50 {
                    last_update.store(now, Ordering::Relaxed);
                    let path_str = path.display().to_string();
                    let weak_clone = progress_weak.clone();
                    glib::MainContext::default().invoke(move || {
                        if let Some(state) = weak_clone.upgrade() {
                            state.lock().unwrap().detection_progress = Some(path_str);
                        }
                    });
                }
            });

            glib::MainContext::default().invoke(move || {
                if let Some(state) = state_weak.upgrade() {
                    let mut s = state.lock().unwrap();
                    s.detection_running = false;
                    s.detection_progress = None;

                    match result {
                        Ok(install) => {
                            s.set_current_game_state(GameState {
                                install_path: Some(install.root_path),
                                detected: true,
                            });
                            s.error_message = None;
                            s.save();
                        }
                        Err(e) => {
                            s.error_message = Some(format!("Detection failed: {}", e));
                        }
                    }
                    drop(s);

                    // Trigger a refresh — we need to locate the panel widget from state
                    // This is handled by the window's refresh mechanism
                }
            });
        });
    }

    fn on_browse_clicked(state: &SharedState, btn: &Button, _container: &GtkBox) {
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
                        s.save();
                    }
                }
            }
            dialog.close();
        });

        dialog.show();
    }

    fn on_download_clicked(state: &SharedState) {
        let state_weak = std::sync::Arc::downgrade(state);
        { state.lock().unwrap().download_progress = Some(0.0); }

        runtime::background().spawn(async move {
            // Download progress callback
            let progress_weak = state_weak.clone();

            let manager = match ogl_core::engine_manager::EngineManager::new() {
                Ok(mgr) => mgr,
                Err(e) => {
                    glib::MainContext::default().invoke(move || {
                        if let Some(state) = state_weak.upgrade() {
                            let mut s = state.lock().unwrap();
                            s.download_progress = None;
                            s.error_message = Some(format!("Failed to resolve engines directory: {}", e));
                        }
                    });
                    return;
                }
            };

            let platform = match ogl_core::engine_manager::EnginePlatform::current() {
                Some(p) => p,
                None => {
                    glib::MainContext::default().invoke(move || {
                        if let Some(state) = state_weak.upgrade() {
                            let mut s = state.lock().unwrap();
                            s.download_progress = None;
                            s.error_message = Some("Unsupported platform for engine download".to_string());
                        }
                    });
                    return;
                }
            };

            let result = manager.install_latest(platform, Some(Box::new(move |current, total| {
                if total > 0 {
                    let progress = current as f64 / total as f64;
                    let weak_clone = progress_weak.clone();
                    glib::MainContext::default().invoke(move || {
                        if let Some(state) = weak_clone.upgrade() {
                            state.lock().unwrap().download_progress = Some(progress);
                        }
                    });
                }
            }))).await;

            match result {
                Ok(install) => {
                    glib::MainContext::default().invoke(move || {
                        if let Some(state) = state_weak.upgrade() {
                            let mut s = state.lock().unwrap();
                            s.download_progress = None;
                            s.config.active_engine = Some(install.version.clone());
                            // Refresh engine list
                            if let Ok(mgr) = ogl_core::engine_manager::EngineManager::new() {
                                s.installed_engines = mgr.list_installed().unwrap_or_default();
                            }
                            s.error_message = None;
                            s.save();
                        }
                    });
                }
                Err(e) => {
                    glib::MainContext::default().invoke(move || {
                        if let Some(state) = state_weak.upgrade() {
                            let mut s = state.lock().unwrap();
                            s.download_progress = None;
                            s.error_message = Some(format!("Download failed: {}", e));
                        }
                    });
                }
            }
        });
    }

    fn on_launch_clicked(state: &SharedState) {
        let s = state.lock().unwrap();
        let game_state = s.current_game_state();
        let gothic_root = match game_state.install_path.as_ref() {
            Some(p) => p.clone(),
            None => return,
        };
        let selected_game = s.selected_game;

        // Find engine executable
        let engine_path = if let Some(ref active_version) = s.config.active_engine {
             s.installed_engines.iter()
                .find(|e| e.version == *active_version)
                .map(|e| e.executable_path.clone())
                .or_else(|| s.installed_engines.first().map(|e| e.executable_path.clone()))
        } else {
            s.installed_engines.first().map(|e| e.executable_path.clone())
        };

        let engine_path = match engine_path {
            Some(p) => p,
            None => {
                drop(s);
                return;
            }
        };
        let mods = Self::mods_for_game(selected_game, &gothic_root);
        if selected_game == ogl_core::install_detector::GothicGame::ChroniclesOfMyrtana && mods.is_empty() {
            drop(s);
            let mut s = state.lock().unwrap();
            s.error_message = Some("Archolos: mod manifest (.ini) not found in System/".to_string());
            return;
        }
        drop(s);

        if cfg!(target_os = "linux") {
            Self::ensure_case_dir(&gothic_root, "System");
            Self::ensure_case_dir(&gothic_root, "Data");
        }

        let state_weak = std::sync::Arc::downgrade(state);
        runtime::background().spawn(async move {
            let executor = ogl_executor::Executor::new(&engine_path);
            if let Err(e) = executor.launch(&gothic_root, &mods).await {
                glib::MainContext::default().invoke(move || {
                    if let Some(state) = state_weak.upgrade() {
                        let mut s = state.lock().unwrap();
                        s.error_message = Some(format!("Launch failed: {}", e));
                    }
                });
            }
        });
    }

    fn mods_for_game(game: ogl_core::install_detector::GothicGame, gothic_root: &std::path::Path) -> Vec<String> {
        if game != ogl_core::install_detector::GothicGame::ChroniclesOfMyrtana {
            return Vec::new();
        }

        let system_dir = match Self::find_system_dir_ci(gothic_root) {
            Some(dir) => dir,
            None => return Vec::new(),
        };
        let primary = system_dir.join("TheChroniclesOfMyrtana.ini");
        if primary.exists() {
            return vec!["TheChroniclesOfMyrtana.ini".to_string()];
        }

        if let Ok(entries) = std::fs::read_dir(&system_dir) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let lower = name.to_lowercase();
                if lower.ends_with(".ini") && (lower.contains("myrtana") || lower.contains("chronicles")) {
                    return vec![name];
                }
            }
        }

        Vec::new()
    }

    fn find_system_dir_ci(root: &std::path::Path) -> Option<std::path::PathBuf> {
        let direct = root.join("System");
        if direct.is_dir() {
            return Some(direct);
        }
        let entries = std::fs::read_dir(root).ok()?;
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            if name == "system" {
                let path = entry.path();
                if path.is_dir() {
                    return Some(path);
                }
            }
        }
        None
    }

    fn ensure_case_dir(root: &std::path::Path, expected: &str) {
        let direct = root.join(expected);
        if direct.exists() {
            return;
        }
        let expected_lower = expected.to_lowercase();
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.to_lowercase() == expected_lower {
                    let target = entry.path();
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::symlink;
                        if let Err(e) = symlink(&target, &direct) {
                            tracing::warn!("Failed to create symlink {} -> {}: {}", direct.display(), target.display(), e);
                        } else {
                            tracing::info!("Created symlink {} -> {}", direct.display(), target.display());
                        }
                    }
                    #[cfg(not(unix))]
                    {
                        let _ = target; // no-op on non-unix
                    }
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_mods_for_archolos_primary_ini() {
        let temp = tempdir().unwrap();
        let system = temp.path().join("System");
        std::fs::create_dir_all(&system).unwrap();
        std::fs::write(system.join("TheChroniclesOfMyrtana.ini"), "dummy").unwrap();

        let mods = GamePanel::mods_for_game(
            ogl_core::install_detector::GothicGame::ChroniclesOfMyrtana,
            temp.path(),
        );
        assert_eq!(mods, vec!["TheChroniclesOfMyrtana.ini".to_string()]);
    }

    #[test]
    fn test_mods_for_archolos_fallback_ini() {
        let temp = tempdir().unwrap();
        let system = temp.path().join("System");
        std::fs::create_dir_all(&system).unwrap();
        std::fs::write(system.join("myrtana_custom.ini"), "dummy").unwrap();

        let mods = GamePanel::mods_for_game(
            ogl_core::install_detector::GothicGame::ChroniclesOfMyrtana,
            temp.path(),
        );
        assert_eq!(mods, vec!["myrtana_custom.ini".to_string()]);
    }

    #[test]
    fn test_find_system_dir_case_insensitive() {
        let temp = tempdir().unwrap();
        let system = temp.path().join("system");
        std::fs::create_dir_all(&system).unwrap();

        let found = GamePanel::find_system_dir_ci(temp.path()).unwrap();
        assert_eq!(found, system);
    }

    #[cfg(unix)]
    #[test]
    fn test_ensure_case_dir_creates_symlink() {
        let temp = tempdir().unwrap();
        let lower = temp.path().join("system");
        std::fs::create_dir_all(&lower).unwrap();

        GamePanel::ensure_case_dir(temp.path(), "System");

        let upper = temp.path().join("System");
        assert!(upper.exists());
        assert!(std::fs::read_link(&upper).is_ok());
    }
}
