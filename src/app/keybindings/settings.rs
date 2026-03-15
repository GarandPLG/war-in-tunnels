use crate::app::{
    App, View,
    keybindings::{Action, event_to_action},
};
use ratatui::crossterm::event::KeyEvent;

pub fn settings_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        match action {
            // Action::Up,
            // Action::Down,
            Action::Quit => app.exit = true,
            Action::Quit2 => app.exit = true,
            Action::Esc => app.view = View::MainMenu,
            Action::Space => {
                app.game_states.settings_state.show_popup =
                    !app.game_states.settings_state.show_popup
            }
            Action::Backspace => {
                if app.game_states.settings_state.show_popup {
                    app.game_states
                        .settings_state
                        .selected_setting_new_value
                        .pop();
                }
            }
            Action::WildCard(c) => {
                if app.game_states.settings_state.show_popup {
                    app.game_states
                        .settings_state
                        .selected_setting_new_value
                        .push(c);
                }
            }
            _ => (),
        }
    }
}
