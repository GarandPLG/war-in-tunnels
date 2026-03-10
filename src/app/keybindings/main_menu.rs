use crate::app::{
    App,
    keybindings::{Action, event_to_action},
};
use ratatui::crossterm::event::KeyEvent;

pub fn main_menu_keybindings(app: &mut App, event: &KeyEvent) {
    if let Some(action) = event_to_action(&event) {
        match action {
            Action::Quit => app.exit = true,
            _ => (),
        }
    }
}
