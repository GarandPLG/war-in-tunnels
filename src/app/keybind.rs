use crate::app::{
  App,
  keybindings::{default_keybindings, main_menu_keybindings, skirmish_keybindings},
  view::View,
};
use ratatui::crossterm::event::KeyEvent;

pub fn handle_keybindings(app: &mut App, key_event: KeyEvent) {
  match app.view {
    View::MainMenu => main_menu_keybindings(app, &key_event),
    View::Skirmish => skirmish_keybindings(app, &key_event),
    _ => default_keybindings(app, &key_event),
  }
}
