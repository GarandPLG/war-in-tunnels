use crate::app::{
    App,
    keybindings::{Action, common_keybindings, event_to_action},
    states::ZoomLevel,
};
use ratatui::crossterm::event::KeyEvent;

pub fn skirmish_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        common_keybindings(app, action);

        let Some(states) = app.states_mut() else {
            return;
        };

        match action {
            Action::Up => states.skirmish.board.focused_cell.move_up(),
            Action::Down => states.skirmish.board.focused_cell.move_down(),
            Action::Left => states.skirmish.board.focused_cell.move_left(),
            Action::Right => states.skirmish.board.focused_cell.move_right(),
            Action::ScrollUp => {
                states.skirmish.board.vertical_offset.prev();
                states.skirmish.board.focused_cell.move_up();
            }
            Action::ScrollDown => {
                states.skirmish.board.vertical_offset.next();
                states.skirmish.board.focused_cell.move_down();
            }
            Action::ScrollLeft => {
                states.skirmish.board.horizontal_offset.prev();
                states.skirmish.board.focused_cell.move_left();
            }
            Action::ScrollRight => {
                states.skirmish.board.horizontal_offset.next();
                states.skirmish.board.focused_cell.move_right();
            }
            Action::ZoomIn => match states.skirmish.board.zoom_level {
                ZoomLevel::ZoomedIn => {}
                ZoomLevel::Default => {
                    states.skirmish.board.zoom_change(ZoomLevel::ZoomedIn);
                    states.skirmish.board.vertical_offset.next();
                }
                ZoomLevel::ZoomedOut => {
                    states.skirmish.board.zoom_change(ZoomLevel::Default);
                    states.skirmish.board.vertical_offset.next();
                    states.skirmish.board.vertical_offset.next();
                }
            },
            Action::ZoomOut => match states.skirmish.board.zoom_level {
                ZoomLevel::ZoomedIn => {
                    states.skirmish.board.zoom_change(ZoomLevel::Default);
                    states.skirmish.board.vertical_offset.prev();
                }
                ZoomLevel::Default => {
                    states.skirmish.board.zoom_change(ZoomLevel::ZoomedOut);
                    states.skirmish.board.vertical_offset.prev();
                    states.skirmish.board.vertical_offset.prev();
                }
                ZoomLevel::ZoomedOut => {}
            },
            _ => (),
        }
    }
}
