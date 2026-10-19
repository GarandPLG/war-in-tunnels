use ratatui::layout::Rect;
use war_in_tunnels::app::states::skirmish_states::{
    BoardState, FocusedCell, MoveFocusedCell, Offset, Players, ZoomLevel,
    structures::{Structure, Structures},
    tasks::Task,
    units::Unit,
};

fn board() -> BoardState {
    BoardState::new(&Rect::new(0, 0, 120, 40), 42, 11, ZoomLevel::Default, false)
}

#[test]
fn map_has_opposing_bases_ore_and_two_miners() {
    let board = board();
    assert_eq!(board.cells.len(), 11);
    assert!(board.cells.iter().all(|row| row.len() == 42));
    for (col, owner, ore_col) in [(2, Players::Player, 1), (39, Players::Enemy, 40)] {
        let cell = board.get_ref_cell(5, col);
        assert!(matches!(cell.get_structure(), Structures::Base(_)));
        assert_eq!(cell.get_structure().get_owner(), owner);
        let unit = cell.get_ref_option_unit().as_ref().unwrap();
        assert_eq!(unit.get_coords(), (5, col));
        assert_eq!(unit.get_owner(), owner);
        assert!(matches!(
            board.get_ref_cell(5, ore_col).get_structure(),
            Structures::Ore(_)
        ));
        assert_eq!(
            board.get_ref_cell(5, ore_col).get_structure().get_owner(),
            Players::Unclaimed
        );
    }
    assert_eq!(
        board
            .cells
            .iter()
            .flatten()
            .filter(|c| c.get_ref_option_unit().is_some())
            .count(),
        2
    );
    assert!(matches!(
        board.get_ref_cell(0, 0).get_structure(),
        Structures::Stone(_)
    ));
}

#[test]
fn focus_cannot_leave_map_even_with_extreme_coordinates() {
    let mut focus = FocusedCell::new(0, 0, 11, 42);
    assert_eq!(focus.move_focused_cell(MoveFocusedCell::Up), (0, 0));
    assert_eq!(focus.move_focused_cell(MoveFocusedCell::Left), (0, 0));
    focus.set_focused_cell((usize::MAX, usize::MAX));
    assert_eq!(focus.get_coords(), (10, 41));
    assert_eq!(focus.move_focused_cell(MoveFocusedCell::Down), (10, 41));
    assert_eq!(focus.move_focused_cell(MoveFocusedCell::Right), (10, 41));
}

#[test]
fn offsets_remain_within_bounds_when_limits_change() {
    let mut offset = Offset::new(Some(100), Some(5));
    assert_eq!(offset.get_value(), 5);
    offset.next();
    assert_eq!(offset.get_value(), 5);
    offset.set_max(2);
    assert_eq!(offset.get_value(), 2);
    offset.set_value(usize::MAX);
    assert_eq!(offset.get_value(), 2);
    for _ in 0..4 {
        offset.prev();
    }
    assert_eq!(offset.get_value(), 0);
}

#[test]
fn confirming_path_queues_task_without_including_start_cell() {
    let mut board = board();
    board.change_focused_cell(MoveFocusedCell::Left);
    board.toggle_marking();
    board.change_focused_cell(MoveFocusedCell::Right);
    board.change_focused_cell(MoveFocusedCell::Down);
    board.toggle_marking();
    let unit = board
        .get_ref_cell(5, 2)
        .get_ref_option_unit()
        .as_ref()
        .unwrap();
    let tasks = unit.get_ref_tasks();
    assert_eq!(tasks.len(), 1);
    assert_eq!(
        tasks[0].get_path().iter().copied().collect::<Vec<_>>(),
        [(5, 3), (6, 3)]
    );
    assert!(!board.marked_cells.marking_cells);
    assert!(board.get_marked_cells().is_empty());
    assert!(board.cells.iter().flatten().all(|c| !c.get_marked()));
}

#[test]
fn cancelling_path_clears_marks_without_queuing_task() {
    let mut board = board();
    board.change_focused_cell(MoveFocusedCell::Left);
    board.toggle_marking();
    board.change_focused_cell(MoveFocusedCell::Right);
    board.marked_cells.marking_cells = false;
    board.end_marking_cells(true);
    assert!(
        board
            .get_ref_cell(5, 2)
            .get_ref_option_unit()
            .as_ref()
            .unwrap()
            .get_ref_tasks()
            .is_empty()
    );
    assert!(board.get_marked_cells().is_empty());
    assert!(board.marked_cells.selected_unit.is_none());
    assert!(board.cells.iter().flatten().all(|c| !c.get_marked()));
}

#[test]
fn backtracking_revisited_cell_preserves_earlier_mark() {
    let mut board = board();
    board.change_focused_cell(MoveFocusedCell::Left);
    board.toggle_marking();
    board.change_focused_cell(MoveFocusedCell::Right);
    board.change_focused_cell(MoveFocusedCell::Left);
    board.pop_marked_cell();
    assert_eq!(board.get_focused_cell().get_coords(), (5, 3));
    assert!(board.get_ref_cell(5, 2).get_marked());
    board.pop_marked_cell();
    board.pop_marked_cell();
    assert_eq!(board.get_focused_cell().get_coords(), (5, 2));
    assert_eq!(board.get_marked_cells().len(), 1);
    assert!(!board.get_ref_cell(5, 3).get_marked());
}

#[test]
fn large_viewport_and_zoom_keep_camera_inside_map() {
    for zoom in [
        ZoomLevel::ZoomedIn,
        ZoomLevel::Default,
        ZoomLevel::ZoomedOut,
    ] {
        let mut board = BoardState::new(&Rect::new(0, 0, 240, 120), 36, 11, zoom, false);
        assert_eq!(board.vertical_offset.get_value(), 0);
        board.horizontal_offset.set_value(usize::MAX);
        board.change_resize(&Rect::new(0, 0, 500, 120), false);
        assert_eq!(board.horizontal_offset.get_value(), 0);
        board.change_zoom(ZoomLevel::ZoomedOut);
        assert!(board.is_focused_cell_visible());
        assert_eq!(board.cells.len(), 11);
    }
}
