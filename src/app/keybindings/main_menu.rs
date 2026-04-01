use crate::app::{
    App, View,
    keybindings::{Action, common_keybindings, event_to_action},
};
use ratatui::crossterm::event::KeyEvent;

pub fn main_menu_keybindings(app: &mut App, event: &KeyEvent) {
    if let Some(action) = event_to_action(&event) {
        common_keybindings(app, action);

        let Some(states) = app.states_mut() else {
            return;
        };

        match action {
            Action::Up => {
                states.main_menu.selected_view =
                    states.main_menu.selected_view.saturating_sub(1).max(1);
            }
            Action::Down => {
                states.main_menu.selected_view =
                    states.main_menu.selected_view.saturating_add(1).min(3);
            }
            Action::Space => {
                let selected_view: usize = states.main_menu.selected_view;

                if selected_view == 1 {
                    app.view = View::Skirmish;
                } else if selected_view == 2 {
                    app.view = View::PerkDecks;
                } else if selected_view == 3 {
                    app.view = View::SkillsConfig;
                }
            }
            _ => (),
        }
    }
}
