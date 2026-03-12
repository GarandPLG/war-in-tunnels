use crate::app::{
    App, View,
    keybindings::{default_view_keybindings, main_menu_keybindings},
};
use ratatui::crossterm::event::KeyEvent;
use std::io::Result;

pub fn handle_keybindings(app: &mut App, key_event: KeyEvent) -> Result<()> {
    match app.view {
        View::MainMenu => main_menu_keybindings(app, &key_event),
        _ => default_view_keybindings(app, &key_event),
    }

    Ok(())
}
