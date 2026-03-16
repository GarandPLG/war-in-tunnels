use crate::app::{
    App, View,
    keybindings::{Action, event_to_action},
};
use ratatui::crossterm::event::KeyEvent;

pub fn main_menu_keybindings(app: &mut App, event: &KeyEvent) {
    if let Some(action) = event_to_action(&event) {
        match action {
            Action::Quit => app.exit = true,
            Action::Quit2 => app.exit = true,
            Action::Up => {
                app.states.main_menu.selected_view =
                    app.states.main_menu.selected_view.saturating_sub(1).max(1);
            }
            Action::Down => {
                app.states.main_menu.selected_view =
                    app.states.main_menu.selected_view.saturating_add(1).min(4);
            }
            Action::Space => {
                let selected_view: usize = app.states.main_menu.selected_view;

                if selected_view == 1 {
                    app.view = View::Skirmish;
                } else if selected_view == 2 {
                    app.view = View::PerkDecks;
                } else if selected_view == 3 {
                    app.view = View::SkillsConfig;
                } else if selected_view == 4 {
                    app.view = View::Settings;
                }
            }
            _ => (),
        }
    }
}
