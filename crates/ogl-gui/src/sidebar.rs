use gtk4::prelude::*;
use gtk4::{ListBox, ListBoxRow, Label, Orientation, Box as GtkBox, SelectionMode};


use ogl_core::GothicGame;
use crate::view_models::{AppUiState, SharedUiState};

/// Build the sidebar ListBox with one row per game.
///
/// When the user clicks a row, `on_game_selected` is called with the new game.
pub fn build_sidebar<F>(state: &SharedUiState, on_game_selected: F) -> ListBox
where
    F: Fn(GothicGame) + 'static,
{
    let list_box = ListBox::new();
    list_box.set_selection_mode(SelectionMode::Single);
    list_box.add_css_class("navigation-sidebar");
    list_box.set_vexpand(true);

    let games = AppUiState::sidebar_games();
    let mut initial_row_idx: i32 = 0;

    for (i, game) in games.iter().enumerate() {
        let row = ListBoxRow::new();
        let hbox = GtkBox::new(Orientation::Horizontal, 8);
        hbox.set_margin_top(8);
        hbox.set_margin_bottom(8);
        hbox.set_margin_start(12);
        hbox.set_margin_end(12);

        // Game icon placeholder (emoji) + name label
        let icon_label = Label::new(Some(game_icon(*game)));
        icon_label.add_css_class("sidebar-icon");

        let name_label = Label::new(Some(game_short_name(*game)));
        name_label.set_hexpand(true);
        name_label.set_halign(gtk4::Align::Start);

        hbox.append(&icon_label);
        hbox.append(&name_label);
        row.set_child(Some(&hbox));
        list_box.append(&row);

        // Mark the initially selected game
        let state_borrow = state.lock().unwrap();
        if *game == state_borrow.selected_game {
            initial_row_idx = i as i32;
        }
    }

    // Select initial row
    if let Some(row) = list_box.row_at_index(initial_row_idx) {
        list_box.select_row(Some(&row));
    }

    // Connect selection change
    let on_selected = std::sync::Arc::new(std::sync::Mutex::new(on_game_selected));
    list_box.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let idx = row.index() as usize;
            let games = AppUiState::sidebar_games();
            if let Some(game) = games.get(idx) {
                (on_selected.lock().unwrap())(*game);
            }
        }
    });

    list_box
}

/// Short names for the sidebar (fit narrow panel).
fn game_short_name(game: GothicGame) -> &'static str {
    match game {
        GothicGame::Gothic1 => "Gothic",
        GothicGame::Gothic2 => "Gothic II",
        GothicGame::Gothic2NotR => "Gothic II: NK",
        GothicGame::ChroniclesOfMyrtana => "Archolos",
        GothicGame::Gothic3 => "Gothic 3",
    }
}

/// Emoji icons as placeholder (can be replaced with actual icons later).
fn game_icon(game: GothicGame) -> &'static str {
    match game {
        GothicGame::Gothic1 => "⚔️",
        GothicGame::Gothic2 => "🏰",
        GothicGame::Gothic2NotR => "🏰",
        GothicGame::ChroniclesOfMyrtana => "📜",
        GothicGame::Gothic3 => "🐉",
    }
}
