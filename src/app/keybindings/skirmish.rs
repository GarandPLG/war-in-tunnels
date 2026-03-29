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
            Action::Up => {
                app.states.skirmish.focused_cell.move_up();

                // if app.states.skirmish.horizontal_offset.get_value() {

                // }
            }
            Action::Down => app.states.skirmish.focused_cell.move_down(),
            Action::Left => app.states.skirmish.focused_cell.move_left(),
            Action::Right => app.states.skirmish.focused_cell.move_right(),
            Action::ScrollUp => app.states.skirmish.vertical_offset.prev(),
            Action::ScrollDown => app.states.skirmish.vertical_offset.next(),
            Action::ScrollLeft => app.states.skirmish.horizontal_offset.prev(),
            Action::ScrollRight => app.states.skirmish.horizontal_offset.next(),
            Action::ZoomIn => match app.states.skirmish.zoom_level {
                ZoomLevel::ZoomedIn => {}
                ZoomLevel::Default => {
                    app.states.skirmish.zoom_level = ZoomLevel::ZoomedIn;
                }
                ZoomLevel::ZoomedOut => {
                    app.states.skirmish.zoom_level = ZoomLevel::Default;
                }
            },
            Action::ZoomOut => match app.states.skirmish.zoom_level {
                ZoomLevel::ZoomedIn => {
                    app.states.skirmish.zoom_level = ZoomLevel::Default;
                }
                ZoomLevel::Default => {
                    app.states.skirmish.zoom_level = ZoomLevel::ZoomedOut;
                }
                ZoomLevel::ZoomedOut => {}
            },
            _ => (),
        }
    }
}
