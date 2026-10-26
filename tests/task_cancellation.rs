use clap::Parser;
use ratatui::{
    Terminal,
    backend::TestBackend,
    buffer::Cell,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::Color,
};
use std::sync::mpsc::{Receiver, Sender, channel};
use war_in_tunnels::{
    app::{
        Action, App, GameStates, View, available_skirmish_actions, handle_keybindings,
        states::skirmish_states::{
            BoardState, MoveFocusedCell,
            structures::{Structure, Structures, Tunnel},
            tasks::{DiggingTask, MovingTask, Task, Tasks},
            units::Unit,
        },
        threads::AudioCmd,
    },
    cli::Cli,
};

fn app() -> App {
    let args: Cli = Cli::try_parse_from(["game", "--disable-audio"]).unwrap();
    let (tx, _rx): (Sender<AudioCmd>, Receiver<AudioCmd>) = channel();
    let mut app: App = App::new(args, tx);
    app.view = View::Skirmish;
    app.window_area = Rect::new(0, 0, 160, 50);
    app.states = Some(GameStates::new(&app.args, &app.window_area));
    app.states_mut()
        .unwrap()
        .skirmish
        .board
        .change_focused_cell(MoveFocusedCell::Left);
    app
}

fn press(app: &mut App, code: KeyCode) -> () {
    handle_keybindings(app, KeyEvent::new(code, KeyModifiers::NONE));
}

fn route_color_count(app: &mut App) -> usize {
    let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(160, 50)).unwrap();
    terminal
        .draw(|frame: &mut ratatui::Frame<'_>| -> () {
            frame.render_widget(app, frame.area());
        })
        .unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .filter(|cell: &&Cell| -> bool {
            matches!(cell.fg, Color::Rgb(128, 0, 128) | Color::Rgb(255, 0, 255))
        })
        .count()
}

#[test]
fn focused_queued_route_is_yellow_until_drawing_a_task() -> () {
    let mut app: App = app();
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.cells[5][3].set_structure(Structures::Tunnel(Tunnel::new()));
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
    board.change_focused_cell(MoveFocusedCell::Right);
    for drawing in [false, true] {
        if drawing {
            press(&mut app, KeyCode::Left);
            press(&mut app, KeyCode::Char(' '));
        }
        let board: &BoardState = &app.states_ref().unwrap().skirmish.board;
        assert_eq!(board.get_focused_cell().get_coords(), (5, 3));
        for stress in [false, true] {
            app.states_mut().unwrap().skirmish.stress_view = stress;
            let mut terminal: Terminal<TestBackend> =
                Terminal::new(TestBackend::new(160, 50)).unwrap();
            terminal
                .draw(|frame: &mut ratatui::Frame<'_>| -> () {
                    frame.render_widget(&mut app, frame.area());
                })
                .unwrap();
            let corner: &Cell = &terminal
                .backend()
                .buffer()
                .content
                .windows(3)
                .find(|cells: &&[Cell]| -> bool {
                    cells[0].symbol() == "╭" && cells[1].symbol() == "D" && cells[2].symbol() == "5"
                })
                .unwrap()[0];
            let expected: Color = if drawing {
                Color::Rgb(128, 0, 128)
            } else {
                Color::Rgb(255, 255, 0)
            };
            assert_eq!(corner.fg, expected);
        }
    }
}

#[test]
fn queued_route_colors_follow_progress_and_cancellation_without_marking_model() -> () {
    let mut app: App = app();
    let baseline: usize = route_color_count(&mut app);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.cells[5][3].set_structure(Structures::Tunnel(Tunnel::new()));
    board.cells[5][4].set_structure(Structures::Tunnel(Tunnel::new()));
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 4)].into())));
    let queued: usize = route_color_count(&mut app);
    assert!(queued > baseline);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    assert!(!board.cells[5][3].get_marked());
    assert!(!board.cells[5][4].get_marked());
    for _ in 0..91 {
        board.advance_turn();
    }
    let remaining: usize = route_color_count(&mut app);
    assert!(remaining > baseline && remaining < queued);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.change_focused_cell(MoveFocusedCell::Right);
    assert!(board.cancel_focused_tasks(true));
    assert_eq!(route_color_count(&mut app), baseline);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.cells[5][3]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 4)].into())));
    for _ in 0..91 {
        board.advance_turn();
    }
    assert_eq!(route_color_count(&mut app), baseline);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.cells[5][39]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 4)].into())));
    assert_eq!(route_color_count(&mut app), baseline);
}

#[test]
fn cancelling_last_preserves_active_work_and_delete_stops_without_restoring_stone() -> () {
    let mut app: App = app();
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    let power: f32 = board.cells[5][2].get_ref_option_unit().get_digging_power();
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Digging(DiggingTask::new([(5, 3)].into(), power)));
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Digging(DiggingTask::new([(5, 4)].into(), power)));
    board.advance_turn();
    let stone: Structures = board.cells[5][3].get_structure();
    assert!(stone.get_durability() < stone.get_max_durability());
    let active: Tasks = board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0].clone();
    press(&mut app, KeyCode::Delete);
    assert_eq!(
        app.states_ref().unwrap().skirmish.board.cells[5][2]
            .get_ref_option_unit()
            .get_ref_tasks()
            .len(),
        2
    );
    press(&mut app, KeyCode::Tab);
    press(&mut app, KeyCode::Backspace);
    let board: &BoardState = &app.states_ref().unwrap().skirmish.board;
    assert_eq!(
        board.cells[5][2]
            .get_ref_option_unit()
            .get_ref_tasks()
            .len(),
        1
    );
    assert_eq!(
        board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0],
        active
    );
    press(&mut app, KeyCode::Delete);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.advance_turn();
    assert!(
        board.cells[5][2]
            .get_ref_option_unit()
            .get_ref_tasks()
            .is_empty()
    );
    assert_eq!(board.cells[5][3].get_structure(), stone);
    assert!(!board.cancel_focused_tasks(true));
}

#[test]
fn cancelling_active_movement_keeps_unit_on_origin_and_discards_progress() -> () {
    let mut app: App = app();
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.cells[5][3].set_structure(Structures::Tunnel(Tunnel::new()));
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
    for _ in 0..40 {
        board.advance_turn();
    }
    assert!(board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0].get_progress() > 0.0);
    assert!(board.cancel_focused_tasks(false));
    for _ in 0..100 {
        board.advance_turn();
    }
    assert!(board.cells[5][2].get_ref_option_unit().is_some());
    assert!(board.cells[5][3].get_ref_option_unit().is_none());
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
    board.advance_turn();
    assert!(board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0].get_progress() < 2.0);
}

#[test]
fn cancellation_rejects_enemy_empty_cells_and_route_drawing() -> () {
    let mut app: App = app();
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
    press(&mut app, KeyCode::Tab);
    assert!(
        available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Delete)
    );
    press(&mut app, KeyCode::Char(' '));
    assert!(
        !app.states_mut()
            .unwrap()
            .skirmish
            .board
            .cancel_focused_tasks(true)
    );
    press(&mut app, KeyCode::Esc);
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    while board.get_focused_cell().get_coords().1 < 39 {
        board.change_focused_cell(MoveFocusedCell::Right);
    }
    board.cells[5][39]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 38)].into())));
    assert!(!board.cancel_focused_tasks(true));
    board.change_focused_cell(MoveFocusedCell::Up);
    assert!(!board.cancel_focused_tasks(true));
}
