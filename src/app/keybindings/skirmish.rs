use crate::app::{
    App,
    keybindings::{Action, common_keybindings, event_to_action},
    states::{
        SkirmishState,
        skirmish_states::{
            BoardState, FocusedCell, MoveFocusedCell, Players, ZoomLevel,
            structures::{Structure, Structures},
            units::{Unit, Units},
        },
    },
};
use ratatui::{crossterm::event::KeyEvent, layout::Rect};

pub fn available_skirmish_actions(state: &SkirmishState) -> Vec<Action> {
    if state.task_view {
        let mut actions: Vec<Action> = vec![
            Action::Tasks,
            Action::Left,
            Action::Right,
            Action::Enter,
            Action::Stress,
            Action::Quit2,
        ];
        if state.selected_task == 0 {
            actions.push(Action::AutoSupports);
        }
        return actions;
    }
    if state.recruitment_view {
        let mut actions: Vec<Action> = vec![
            Action::Recruitment,
            Action::Left,
            Action::Right,
            Action::Stress,
            Action::Quit2,
        ];
        if state.can_recruit_miner(state.board.get_focused_cell().get_coords(), Players::Player) {
            actions.push(Action::Enter);
        }
        return actions;
    }
    if state.building_view {
        let mut actions: Vec<Action> = vec![
            Action::Building,
            Action::Left,
            Action::Right,
            Action::Stress,
            Action::Quit2,
        ];
        let coords: (usize, usize) = state.board.get_focused_cell().get_coords();
        if state.selected_building == 1 {
            if state.can_build_support(coords, Players::Player) {
                actions.push(Action::Enter);
            }
            if state.can_manage_support(coords, Players::Player)
                && matches!(state.board.cells[coords.0][coords.1].get_structure(), Structures::Tunnel(tunnel) if tunnel.has_support() && tunnel.support_work().is_none())
            {
                actions.push(Action::Delete);
            }
            return actions;
        }
        if let Structures::MineConstruction(site) =
            state.board.get_ref_cell(coords.0, coords.1).get_structure()
            && site.get_owner() == Players::Player
        {
            actions.push(Action::Delete);
        } else if state.can_start_mine_construction(coords, Players::Player) {
            actions.push(Action::Enter);
        }
        return actions;
    }
    let mut actions: Vec<Action> = vec![Action::Up, Action::Down, Action::Left, Action::Right];
    if state.board.marked_cells.marking_cells {
        actions.extend([
            Action::Space,
            Action::Backspace,
            Action::Delete,
            Action::Esc,
            Action::Tab,
            Action::Quit2,
            Action::Stress,
            Action::Tasks,
        ]);
        if !state.board.movement_only {
            actions.push(Action::AutoSupports);
        }
        return actions;
    }
    actions.extend([
        Action::ScrollUp,
        Action::ScrollDown,
        Action::ScrollLeft,
        Action::ScrollRight,
        Action::Tab,
        Action::Building,
        Action::Mute,
        Action::Quit,
        Action::Esc,
        Action::Stress,
    ]);
    if state.board.zoom_level != ZoomLevel::ZoomedIn {
        actions.push(Action::ZoomIn);
    }
    if state.board.zoom_level != ZoomLevel::ZoomedOut {
        actions.push(Action::ZoomOut);
    }
    let cell: &FocusedCell = state.board.get_focused_cell();
    let (row, col): (usize, usize) = cell.get_coords();
    if matches!(state.board.get_ref_cell(row, col).get_structure(), Structures::Base(base) if base.get_owner() == Players::Player)
    {
        actions.push(Action::Recruitment);
    }
    let unit: &Option<Units> = state.board.get_ref_cell(row, col).get_ref_option_unit();
    if unit.get_owner() == Players::Player && unit.get_can_dig() {
        actions.push(Action::Space);
    }
    if state.side_panel && unit.get_owner() == Players::Player && !unit.get_ref_tasks().is_empty() {
        actions.extend([Action::Backspace, Action::Delete]);
    }
    actions
}

pub fn skirmish_keybindings(app: &mut App, key_event: &KeyEvent) -> () {
    let window_area: Rect = app.window_area;
    if let Some(action) = event_to_action(key_event) {
        let Some(states) = app.states_mut() else {
            common_keybindings(app, action);
            return;
        };
        if action == Action::Stress {
            states.skirmish.stress_view = !states.skirmish.stress_view;
            return;
        }
        if states.skirmish.task_view {
            match action {
                Action::Tasks | Action::Enter => {
                    states.skirmish.task_view = false;
                    if action == Action::Enter {
                        states
                            .skirmish
                            .board
                            .configure_selected_task(states.skirmish.selected_task == 1);
                    }
                    states
                        .skirmish
                        .board
                        .change_resize(&window_area, states.skirmish.side_panel);
                }
                Action::Left => states.skirmish.cycle_popup_option(false),
                Action::Right => states.skirmish.cycle_popup_option(true),
                Action::AutoSupports if states.skirmish.selected_task == 0 => {
                    states.skirmish.board.auto_supports = !states.skirmish.board.auto_supports
                }
                Action::Esc => (),
                _ => common_keybindings(app, action),
            }
            return;
        }
        if states.skirmish.recruitment_view {
            match action {
                Action::Recruitment => states.skirmish.recruitment_view = false,
                Action::Left => states.skirmish.cycle_popup_option(false),
                Action::Right => states.skirmish.cycle_popup_option(true),
                Action::Esc => (),
                Action::Enter => {
                    let coords: (usize, usize) =
                        states.skirmish.board.get_focused_cell().get_coords();
                    states.skirmish.recruit_miner(coords, Players::Player);
                }
                _ => common_keybindings(app, action),
            }
            return;
        }
        if states.skirmish.building_view {
            match action {
                Action::Building => states.skirmish.building_view = false,
                Action::Left => states.skirmish.cycle_popup_option(false),
                Action::Right => states.skirmish.cycle_popup_option(true),
                Action::Esc => (),
                Action::Enter => {
                    let coords: (usize, usize) =
                        states.skirmish.board.get_focused_cell().get_coords();
                    if states.skirmish.selected_building == 1 {
                        states.skirmish.build_support(coords, Players::Player);
                    } else {
                        states
                            .skirmish
                            .start_mine_construction(coords, Players::Player);
                    }
                }
                Action::Delete => {
                    let coords: (usize, usize) =
                        states.skirmish.board.get_focused_cell().get_coords();
                    if states.skirmish.selected_building == 1 {
                        states.skirmish.remove_support(coords, Players::Player);
                    } else {
                        states
                            .skirmish
                            .cancel_mine_construction(coords, Players::Player);
                    }
                }
                _ => common_keybindings(app, action),
            }
            return;
        }
        if action == Action::Esc && states.skirmish.board.marked_cells.marking_cells {
            states.skirmish.board.marked_cells.marking_cells = false;
            states.skirmish.board.end_marking_cells(true);
            return;
        }
        let available: bool = available_skirmish_actions(&states.skirmish).contains(&action);
        common_keybindings(app, action);
        if !available {
            return;
        }

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
            Action::Space => {
                if !board.marked_cells.marking_cells {
                    board.movement_only = false;
                }
                board.toggle_marking();
            }
            Action::Building => states.skirmish.building_view = true,
            Action::Recruitment => states.skirmish.recruitment_view = true,
            Action::Tasks => {
                states.skirmish.task_view = true;
                board.change_resize(&window_area, true);
            }
            Action::AutoSupports => board.auto_supports = !board.auto_supports,
            Action::Backspace if board.marked_cells.marking_cells => board.pop_marked_cell(),
            Action::Backspace => {
                board.cancel_focused_tasks(false);
            }
            Action::Delete if !board.marked_cells.marking_cells => {
                board.cancel_focused_tasks(true);
            }
            Action::Delete => {
                board.marked_cells.marking_cells = false;
                board.end_marking_cells(true)
            }
            Action::Tab => {
                states.skirmish.side_panel = !states.skirmish.side_panel;
                board.change_resize(&window_area, states.skirmish.side_panel);
            }
            _ => (),
        }
    }
}
