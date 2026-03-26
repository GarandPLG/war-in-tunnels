use crate::app::{
    App, View,
    keybindings::{Action, event_to_action},
};
use ratatui::crossterm::event::KeyEvent;

pub fn skirmish_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        match action {
            Action::ScrollUp => app.states.skirmish.vertical_scrollbar.prev(),
            Action::ScrollDown => app.states.skirmish.vertical_scrollbar.next(),
            Action::ScrollLeft => app.states.skirmish.horizontal_scrollbar.prev(),
            Action::ScrollRight => app.states.skirmish.horizontal_scrollbar.next(),
            Action::Quit => app.exit = true,
            Action::Quit2 => app.exit = true,
            Action::Esc => app.view = View::MainMenu,
            _ => (),
        }
    }
}
