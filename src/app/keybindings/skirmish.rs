use crate::app::{
    App,
    keybindings::{Action, common_keybindings, event_to_action},
    states::skirmish_states::{BoardState, MoveFocusedCell, ZoomLevel},
};
use ratatui::{crossterm::event::KeyEvent, layout::Rect};

pub fn skirmish_keybindings(app: &mut App, key_event: &KeyEvent) {
    if let Some(action) = event_to_action(&key_event) {
        common_keybindings(app, action);

        let window_area: Rect = app.window_area;

        let Some(states) = app.states_mut() else {
            return;
        };

        let board: &mut BoardState = &mut states.skirmish.board;

        match action {
            Action::Up => {
                board.change_focused_cell(MoveFocusedCell::Up);
                if !board.is_focused_cell_visible() {
                    board.vertical_offset.prev();
                }
            }
            Action::Down => {
                board.change_focused_cell(MoveFocusedCell::Down);
                if !board.is_focused_cell_visible() {
                    board.vertical_offset.next();
                }
            }
            Action::Left => {
                board.change_focused_cell(MoveFocusedCell::Left);
                if !board.is_focused_cell_visible() {
                    board.horizontal_offset.prev();
                }
            }
            Action::Right => {
                board.change_focused_cell(MoveFocusedCell::Right);
                if !board.is_focused_cell_visible() {
                    board.horizontal_offset.next();
                }
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
                    board.change_zoom(ZoomLevel::ZoomedIn);
                    board.vertical_offset.next();
                }
                ZoomLevel::ZoomedOut => {
                    board.change_zoom(ZoomLevel::Default);
                    board.vertical_offset.next();
                    board.vertical_offset.next();
                }
            },
            Action::ZoomOut => match board.zoom_level {
                ZoomLevel::ZoomedIn => {
                    board.change_zoom(ZoomLevel::Default);
                    board.vertical_offset.prev();
                }
                ZoomLevel::Default => {
                    board.change_zoom(ZoomLevel::ZoomedOut);
                    board.vertical_offset.prev();
                    board.vertical_offset.prev();
                }
                ZoomLevel::ZoomedOut => {}
            },
            Action::Space => board.toggle_marking(),
            Action::Backspace => board.undo_marked_cell(),
            Action::Delete => {
                board.marking_cells = false;
                board.clear_marked_cells()
            }
            Action::Tab => {
                states.skirmish.side_panel = !states.skirmish.side_panel;
                board.change_resize(&window_area, states.skirmish.side_panel);
            }
            _ => (),
        }
    }
}
