use crate::app::{
    App,
    keybindings::{Action, common_keybindings, event_to_action},
    states::{
        ZoomLevel,
        skirmish_states::{BoardState, MoveFocusedCell},
    },
};
use ratatui::crossterm::event::KeyEvent;

pub fn skirmish_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        common_keybindings(app, action);

        let Some(states) = app.states_mut() else {
            return;
        };

        let board: &mut BoardState = &mut states.skirmish.board;

        match action {
            Action::Up => {
                board.change_focused_cell(MoveFocusedCell::Up);
            }
            Action::Down => {
                board.change_focused_cell(MoveFocusedCell::Down);
            }
            Action::Left => {
                board.change_focused_cell(MoveFocusedCell::Left);
            }
            Action::Right => {
                board.change_focused_cell(MoveFocusedCell::Right);
            }
            Action::ScrollUp => {
                board.vertical_offset.prev();
                board.change_focused_cell(MoveFocusedCell::Up);
            }
            Action::ScrollDown => {
                board.vertical_offset.next();
                board.change_focused_cell(MoveFocusedCell::Down);
            }
            Action::ScrollLeft => {
                board.horizontal_offset.prev();
                board.change_focused_cell(MoveFocusedCell::Left);
            }
            Action::ScrollRight => {
                board.horizontal_offset.next();
                board.change_focused_cell(MoveFocusedCell::Right);
            }
            Action::ZoomIn => match board.zoom_level {
                ZoomLevel::ZoomedIn => {}
                ZoomLevel::Default => {
                    board.zoom_change(ZoomLevel::ZoomedIn);
                    board.vertical_offset.next();
                }
                ZoomLevel::ZoomedOut => {
                    board.zoom_change(ZoomLevel::Default);
                    board.vertical_offset.next();
                    board.vertical_offset.next();
                }
            },
            Action::ZoomOut => match board.zoom_level {
                ZoomLevel::ZoomedIn => {
                    board.zoom_change(ZoomLevel::Default);
                    board.vertical_offset.prev();
                }
                ZoomLevel::Default => {
                    board.zoom_change(ZoomLevel::ZoomedOut);
                    board.vertical_offset.prev();
                    board.vertical_offset.prev();
                }
                ZoomLevel::ZoomedOut => {}
            },
            _ => (),
        }
    }
}
