use crate::app::{
    App, View,
    keybindings::{Action, event_to_action},
};
use ratatui::crossterm::event::KeyEvent;

pub fn settings_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        match action {
            Action::Quit => app.exit = true,
            Action::Quit2 => app.exit = true,
            Action::Esc => app.view = View::MainMenu,
            _ => (),
        }
    }
}
