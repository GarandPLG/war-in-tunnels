use crate::app::{
    App, View,
    keybindings::{Action, event_to_action, main_menu_keybindings},
};
use ratatui::crossterm::event::KeyEvent;
use std::io::Result;

pub fn handle_keybindings(app: &mut App, key_event: KeyEvent) -> Result<()> {
    match app.view {
        View::MainMenu => main_menu_keybindings(app, &key_event),
        _ => {
            if let Some(action) = event_to_action(&key_event) {
                match action {
                    Action::Quit => app.exit = true,
                    Action::Quit2 => app.exit = true,
                    Action::Esc => app.view = View::MainMenu,
                    _ => (),
                }
            }
        }
    }

    Ok(())
}
