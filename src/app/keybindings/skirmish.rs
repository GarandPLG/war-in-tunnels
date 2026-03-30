use crate::app::{
    App,
    keybindings::{Action, common_keybindings, event_to_action},
    states::ZoomLevel,
};
use ratatui::crossterm::event::KeyEvent;

pub fn skirmish_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        common_keybindings(app, action);
        match action {
            Action::Up => app.states.skirmish.focused_cell.move_up(),
            Action::Down => app.states.skirmish.focused_cell.move_down(),
            Action::Left => app.states.skirmish.focused_cell.move_left(),
            Action::Right => app.states.skirmish.focused_cell.move_right(),
            Action::ScrollUp => {
                app.states.skirmish.vertical_offset.prev();
                app.states.skirmish.focused_cell.move_up();
            }
            Action::ScrollDown => {
                app.states.skirmish.vertical_offset.next();
                app.states.skirmish.focused_cell.move_down();
            }
            Action::ScrollLeft => {
                app.states.skirmish.horizontal_offset.prev();
                app.states.skirmish.focused_cell.move_left();
            }
            Action::ScrollRight => {
                app.states.skirmish.horizontal_offset.next();
                app.states.skirmish.focused_cell.move_right();
            }
            Action::ZoomIn => match app.states.skirmish.zoom_level {
                ZoomLevel::ZoomedIn => {}
                ZoomLevel::Default => {
                    app.states.skirmish.zoom_level = ZoomLevel::ZoomedIn;
                    app.states.skirmish.vertical_offset.next();
                }
                ZoomLevel::ZoomedOut => {
                    app.states.skirmish.zoom_level = ZoomLevel::Default;
                    app.states.skirmish.vertical_offset.next();
                    app.states.skirmish.vertical_offset.next();
                }
            },
            Action::ZoomOut => match app.states.skirmish.zoom_level {
                ZoomLevel::ZoomedIn => {
                    app.states.skirmish.zoom_level = ZoomLevel::Default;
                    app.states.skirmish.vertical_offset.prev();
                }
                ZoomLevel::Default => {
                    app.states.skirmish.zoom_level = ZoomLevel::ZoomedOut;
                    app.states.skirmish.vertical_offset.prev();
                    app.states.skirmish.vertical_offset.prev();
                }
                ZoomLevel::ZoomedOut => {}
            },
            _ => (),
        }
    }
}
