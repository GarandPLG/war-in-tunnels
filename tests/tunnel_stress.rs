use clap::Parser;
use ratatui::{
    Terminal,
    backend::TestBackend,
    buffer::{Buffer, Cell},
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::Rect,
    style::Color,
    widgets::Widget,
};
use std::sync::mpsc::{Receiver, Sender, channel};
use war_in_tunnels::{
    app::{
        Action, App, GameStates, View, available_skirmish_actions, handle_keybindings,
        states::{
            SkirmishState,
            skirmish_states::{
                BoardState, CellSizes, Players, Resources, ZoomLevel,
                structures::{CollapsedTunnel, Structure, Structures, Tunnel},
                tasks::{DiggingTask, MovingTask, Task, Tasks},
                units::{MinerUnit, Unit, Units},
            },
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
    app
}

fn put_tunnel(board: &mut BoardState, coords: (usize, usize), supported: bool) -> () {
    let mut tunnel: Tunnel = Tunnel::new();
    tunnel.set_support(supported);
    board.cells[coords.0][coords.1].set_structure(Structures::Tunnel(tunnel));
}

fn miner(board: &mut BoardState, coords: (usize, usize), owner: Players) -> () {
    *board.cells[coords.0][coords.1].get_mut_option_unit() =
        Some(Units::Miner(MinerUnit::new(owner, coords)));
}

fn press(app: &mut App, code: KeyCode) -> () {
    handle_keybindings(app, KeyEvent::new(code, KeyModifiers::NONE));
}

fn finish_support_work(game: &mut SkirmishState) -> () {
    for _ in 0..Tunnel::SUPPORT_WORK_TICKS {
        game.advance_support_work();
    }
}

#[test]
fn support_work_takes_three_seconds_freezes_worker_and_pauses_without_builder() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    put_tunnel(&mut game.board, (5, 3), false);
    game.board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
    assert!(game.build_support((5, 3), Players::Player));
    for _ in 1..Tunnel::SUPPORT_WORK_TICKS {
        game.tick_update();
    }
    assert_eq!(game.board.cells[5][3].get_structure().get_level(), ' ');
    assert_eq!(
        game.board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0].get_progress(),
        0.0
    );
    game.tick_update();
    assert_eq!(game.board.cells[5][3].get_structure().get_level(), 'T');
    assert!(game.board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0].get_progress() > 0.0);
    assert!(game.remove_support((5, 3), Players::Player));
    for _ in 0..45 {
        game.tick_update();
    }
    game.board.cells[5][2].get_mut_option_unit().take();
    let paused: Structures = game.board.cells[5][3].get_structure();
    for _ in 0..20 {
        game.tick_update();
    }
    assert_eq!(game.board.cells[5][3].get_structure(), paused);
    assert_eq!(paused.get_level(), 'T');
    miner(&mut game.board, (5, 2), Players::Player);
    for _ in 45..Tunnel::SUPPORT_WORK_TICKS {
        game.tick_update();
    }
    assert_eq!(game.board.cells[5][3].get_structure().get_level(), ' ');
}

#[test]
fn one_miner_cannot_work_on_two_supports_simultaneously() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    put_tunnel(&mut game.board, (5, 3), false);
    put_tunnel(&mut game.board, (5, 4), false);
    miner(&mut game.board, (5, 3), Players::Player);
    assert!(game.build_support((5, 3), Players::Player));
    assert!(game.build_support((5, 4), Players::Player));
    finish_support_work(game);
    assert_eq!(game.board.cells[5][3].get_structure().get_level(), 'T');
    let Structures::Tunnel(tunnel) = game.board.cells[5][4].get_structure() else {
        panic!("tunnel missing");
    };
    assert_eq!(tunnel.support_progress(), Some(0));
    finish_support_work(game);
    assert_eq!(game.board.cells[5][4].get_structure().get_level(), 'T');
}

#[test]
fn stress_map_renders_stone_normally_at_every_zoom() -> () {
    let app: App = app();
    for zoom in [
        ZoomLevel::Default,
        ZoomLevel::ZoomedIn,
        ZoomLevel::ZoomedOut,
    ] {
        for (selected, marked) in [(false, false), (false, true), (true, false), (true, true)] {
            let area: Rect = Rect::new(
                0,
                0,
                zoom.get_cell_size(CellSizes::Width) as u16,
                zoom.get_cell_size(CellSizes::Height) as u16,
            );
            let mut normal: Buffer = Buffer::empty(area);
            let mut stress: Buffer = Buffer::empty(area);
            app.states_ref().unwrap().skirmish.board.cells[5][4]
                .clone()
                .set_zoom_level(zoom)
                .set_selected(selected)
                .set_marked(selected && marked)
                .clone()
                .render(area, &mut normal);
            app.states_ref().unwrap().skirmish.board.cells[5][4]
                .clone()
                .set_zoom_level(zoom)
                .set_selected(selected)
                .set_marked(marked)
                .render_stress(area, &mut stress);
            assert_eq!(normal, stress);
            assert!(
                !stress
                    .content
                    .iter()
                    .any(|cell: &Cell| -> bool { cell.symbol() == "%" })
            );
        }
    }
}

#[test]
fn task_key_requires_space_selection_and_moving_choice_resets_invalid_draft() -> () {
    let mut app: App = app();
    press(&mut app, KeyCode::Left);
    assert!(
        !available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Tasks)
    );
    press(&mut app, KeyCode::Char('t'));
    assert!(!app.states_ref().unwrap().skirmish.task_view);
    press(&mut app, KeyCode::Char(' '));
    assert!(
        available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Tasks)
    );
    press(&mut app, KeyCode::Right);
    press(&mut app, KeyCode::Char('t'));
    assert!(app.states_ref().unwrap().skirmish.task_view);
    press(&mut app, KeyCode::Right);
    press(&mut app, KeyCode::Enter);
    let board: &BoardState = &app.states_ref().unwrap().skirmish.board;
    assert!(board.marked_cells.marking_cells && board.movement_only);
    assert_eq!(board.get_focused_cell().get_coords(), (5, 2));
    assert_eq!(board.get_marked_cells().len(), 1);
    assert!(
        board.cells[5][2]
            .get_ref_option_unit()
            .get_ref_tasks()
            .is_empty()
    );
    press(&mut app, KeyCode::Esc);
    assert!(
        !available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Tasks)
    );
}

#[test]
fn stress_follows_cardinal_distance_from_base_and_caps_at_100() -> () {
    let mut app: App = app();
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    for col in 3..=9 {
        put_tunnel(board, (5, col), false);
    }
    put_tunnel(board, (4, 3), false);
    put_tunnel(board, (6, 3), false);
    board.recalculate_stress();
    for (col, stress) in [(3, 20), (4, 40), (5, 60), (6, 80), (7, 100), (9, 100)] {
        assert_eq!(board.cells[5][col].get_structure().get_stress(), stress);
    }
    assert_eq!(board.cells[4][3].get_structure().get_stress(), 40);
    assert_eq!(board.cells[6][3].get_structure().get_stress(), 40);
}

#[test]
fn support_stabilizes_all_four_directions_and_has_level_marker() -> () {
    let mut app: App = app();
    let board: &mut BoardState = &mut app.states_mut().unwrap().skirmish.board;
    put_tunnel(board, (5, 12), true);
    for coords in [(4, 12), (6, 12), (5, 11), (5, 13)] {
        put_tunnel(board, coords, false);
    }
    board.recalculate_stress();
    assert_eq!(board.cells[5][12].get_structure().get_level(), 'T');
    assert_eq!(board.cells[5][12].get_structure().get_stress(), 20);
    for (row, col) in [(4, 12), (6, 12), (5, 11), (5, 13)] {
        assert_eq!(board.cells[row][col].get_structure().get_stress(), 40);
    }
    assert_eq!(board.resolve_collapses(), 0);
}

#[test]
fn removal_causes_fatal_cascade_and_preserves_safe_tunnels() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    for col in 3..=9 {
        put_tunnel(&mut game.board, (5, col), col == 6);
    }
    miner(&mut game.board, (5, 6), Players::Player);
    miner(&mut game.board, (5, 8), Players::Player);
    miner(&mut game.board, (5, 9), Players::Enemy);
    game.board.resolve_collapses();
    assert!(game.remove_support((5, 6), Players::Player));
    for _ in 1..Tunnel::SUPPORT_WORK_TICKS {
        game.advance_support_work();
    }
    assert_eq!(game.board.cells[5][6].get_structure().get_level(), 'T');
    assert!(game.board.cells[5][9].get_ref_option_unit().is_some());
    game.advance_support_work();
    for col in 7..=9 {
        let structure: Structures = game.board.cells[5][col].get_structure();
        assert!(matches!(structure, Structures::CollapsedTunnel(_)));
        assert_eq!(structure.get_durability(), 750);
        assert!(game.board.cells[5][col].get_ref_option_unit().is_none());
        assert!(
            !structure.can_enter(
                game.board.cells[5][6]
                    .get_ref_option_unit()
                    .as_ref()
                    .unwrap()
            )
        );
    }
    assert!(game.board.cells[5][6].get_ref_option_unit().is_some());
    assert_eq!(game.board.cells[5][6].get_structure().get_stress(), 80);
    assert_eq!(game.board.resolve_collapses(), 0);
}

#[test]
fn alternate_support_prevents_collapse_after_removal() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    for col in 3..=10 {
        put_tunnel(&mut game.board, (5, col), col == 6 || col == 9);
    }
    miner(&mut game.board, (5, 6), Players::Player);
    assert!(game.remove_support((5, 6), Players::Player));
    finish_support_work(game);
    assert!(matches!(
        game.board.cells[5][8].get_structure(),
        Structures::Tunnel(_)
    ));
    assert_eq!(game.board.cells[5][8].get_structure().get_stress(), 40);
}

#[test]
fn supports_charge_atomically_require_local_builder_and_do_not_refund_removal() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    put_tunnel(&mut game.board, (5, 3), false);
    put_tunnel(&mut game.board, (5, 15), false);
    let before: Resources = game.player_resources;
    assert!(!game.build_support((usize::MAX, 0), Players::Player));
    assert!(!game.build_support((5, 15), Players::Player));
    assert!(!game.build_support((5, 3), Players::Enemy));
    assert_eq!(game.player_resources, before);
    game.player_resources = Resources::new(9, 100);
    assert!(!game.build_support((5, 3), Players::Player));
    assert_eq!(game.player_resources, Resources::new(9, 100));
    game.player_resources = Resources::new(10, 2);
    assert!(game.build_support((5, 3), Players::Player));
    assert_eq!(game.player_resources, Resources::new(0, 0));
    assert!(!game.build_support((5, 3), Players::Player));
    assert!(!game.remove_support((5, 3), Players::Player));
    finish_support_work(game);
    assert!(game.remove_support((5, 3), Players::Player));
    assert_eq!(game.player_resources, Resources::new(0, 0));
    assert!(!game.remove_support((5, 3), Players::Player));
}

#[test]
fn support_building_tab_builds_and_removes_upgrade_using_local_miner() -> () {
    let mut app: App = app();
    put_tunnel(&mut app.states_mut().unwrap().skirmish.board, (5, 3), false);
    press(&mut app, KeyCode::Char('b'));
    press(&mut app, KeyCode::Right);
    assert_eq!(app.states_ref().unwrap().skirmish.selected_building, 1);
    press(&mut app, KeyCode::Enter);
    let game: &SkirmishState = &app.states_ref().unwrap().skirmish;
    assert_eq!(game.board.cells[5][3].get_structure().get_level(), ' ');
    finish_support_work(&mut app.states_mut().unwrap().skirmish);
    let game: &SkirmishState = &app.states_ref().unwrap().skirmish;
    assert_eq!(game.board.cells[5][3].get_structure().get_level(), 'T');
    assert_eq!(game.player_resources, Resources::new(40, 23));
    press(&mut app, KeyCode::Delete);
    assert_eq!(
        app.states_ref().unwrap().skirmish.board.cells[5][3]
            .get_structure()
            .get_level(),
        'T'
    );
    finish_support_work(&mut app.states_mut().unwrap().skirmish);
    assert_eq!(
        app.states_ref().unwrap().skirmish.board.cells[5][3]
            .get_structure()
            .get_level(),
        ' '
    );
    assert_eq!(
        app.states_ref().unwrap().skirmish.player_resources,
        Resources::new(40, 23)
    );
    assert!(app.states_ref().unwrap().skirmish.building_view);
}

#[test]
fn rubble_requires_less_work_than_stone_and_percent_is_rendered() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    game.board.cells[5][3].set_structure(Structures::CollapsedTunnel(CollapsedTunnel::new()));
    game.board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Digging(DiggingTask::new([(5, 3)].into(), 25.0)));
    game.tick_update();
    assert_eq!(game.board.cells[5][3].get_structure().get_durability(), 500);
    let area: Rect = Rect::new(0, 0, 9, 5);
    let mut buffer: Buffer = Buffer::empty(area);
    game.board.cells[5][3].clone().render(area, &mut buffer);
    let text: String = buffer
        .content
        .iter()
        .map(|cell: &Cell| -> &str { cell.symbol() })
        .collect();
    assert!(text.contains("66%"));
    for _ in 0..2 {
        game.tick_update();
    }
    assert!(game.board.cells[5][3].get_ref_option_unit().is_some());
    assert!(matches!(
        game.board.cells[5][3].get_structure(),
        Structures::Tunnel(_)
    ));
}

#[test]
fn automatic_supports_are_spaced_at_last_safe_tiles_and_do_not_waste_final_support() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    game.player_resources = Resources::new(1000, 1000);
    game.board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Digging(DiggingTask::new(
            (3..=12)
                .map(|col: usize| -> (usize, usize) { (5, col) })
                .collect(),
            100.0,
        )));
    for _ in 0..400 {
        game.tick_update();
        if game.board.cells[5][12].get_ref_option_unit().is_some() {
            break;
        }
    }
    assert!(game.board.cells[5][12].get_ref_option_unit().is_some());
    assert_eq!(game.player_resources, Resources::new(985, 996));
    assert!(game.turn_counter >= u64::from(2 * Tunnel::SUPPORT_WORK_TICKS));
    for col in 3..=12 {
        let Structures::Tunnel(tunnel) = game.board.cells[5][col].get_structure() else {
            panic!("collapsed at {col}");
        };
        assert_eq!(tunnel.has_support(), col == 6 || col == 9);
        assert!(tunnel.get_stress() < 100);
    }
}

#[test]
fn disabled_auto_supports_or_missing_funds_kills_miner_at_fifth_tile() -> () {
    for (automatic, metal) in [(false, 100), (true, 0)] {
        let mut app: App = app();
        let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
        game.player_resources = Resources::new(100, metal);
        let mut task: DiggingTask = DiggingTask::new(
            (3..=9)
                .map(|col: usize| -> (usize, usize) { (5, col) })
                .collect(),
            100.0,
        );
        task.set_auto_supports(automatic);
        game.board.cells[5][2]
            .get_mut_option_unit()
            .set_task(Tasks::Digging(task));
        for _ in 0..5 {
            game.tick_update();
        }
        assert!(matches!(
            game.board.cells[5][7].get_structure(),
            Structures::CollapsedTunnel(_)
        ));
        assert!(game.board.cells[5][7].get_ref_option_unit().is_none());
        assert_eq!(game.player_resources, Resources::new(100, metal));
    }
}

#[test]
fn rubble_can_be_dug_again_and_auto_setting_survives_task_conversion() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    game.board.cells[5][3].set_structure(Structures::CollapsedTunnel(CollapsedTunnel::new()));
    let mut task: MovingTask = MovingTask::new([(5, 3)].into());
    task.set_auto_supports(false);
    let dig: DiggingTask = DiggingTask::from_moving(task, 100.0);
    assert!(!dig.auto_supports());
    assert!(!MovingTask::from_digging(dig.clone()).auto_supports());
    game.board.cells[5][2]
        .get_mut_option_unit()
        .set_task(Tasks::Digging(dig));
    game.tick_update();
    assert!(matches!(
        game.board.cells[5][3].get_structure(),
        Structures::Tunnel(_)
    ));
    assert!(game.board.cells[5][3].get_ref_option_unit().is_some());
}

#[test]
fn stress_view_hides_all_other_text_and_colors_but_keeps_focused_color() -> () {
    let mut app: App = app();
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    put_tunnel(&mut game.board, (5, 3), true);
    miner(&mut game.board, (5, 3), Players::Player);
    game.board.recalculate_stress();
    for zoom in [
        ZoomLevel::Default,
        ZoomLevel::ZoomedIn,
        ZoomLevel::ZoomedOut,
    ] {
        let area: Rect = Rect::new(
            0,
            0,
            zoom.get_cell_size(CellSizes::Width) as u16,
            zoom.get_cell_size(CellSizes::Height) as u16,
        );
        let mut buffer: Buffer = Buffer::empty(area);
        game.board.cells[5][3]
            .clone()
            .set_zoom_level(zoom)
            .set_selected(false)
            .set_marked(true)
            .render_stress(area, &mut buffer);
        let text: String = buffer
            .content
            .iter()
            .map(|cell: &Cell| -> &str { cell.symbol() })
            .collect();
        assert!(text.contains("20%"));
        assert!(!text.contains('M') && !text.contains('T'));
        assert!(!buffer.content.iter().any(|cell: &Cell| -> bool {
            matches!(cell.fg, Color::Magenta | Color::LightMagenta | Color::Blue)
        }));
        game.board.cells[5][3]
            .clone()
            .set_selected(true)
            .render_stress(area, &mut buffer);
        assert_eq!(buffer[(0, 0)].fg, Color::LightYellow);
        game.board.cells[5][3]
            .clone()
            .set_selected(true)
            .set_marked(true)
            .render_stress(area, &mut buffer);
        assert_eq!(buffer[(0, 0)].fg, Color::Magenta);
    }
}

#[test]
fn task_sidebar_selects_auto_supports_and_captures_setting_in_queued_task() -> () {
    let mut app: App = app();
    press(&mut app, KeyCode::Left);
    press(&mut app, KeyCode::Char(' '));
    let coords: (usize, usize) = app
        .states_ref()
        .unwrap()
        .skirmish
        .board
        .get_focused_cell()
        .get_coords();
    press(&mut app, KeyCode::Char('t'));
    assert!(app.states_ref().unwrap().skirmish.task_view);
    press(&mut app, KeyCode::Char('a'));
    press(&mut app, KeyCode::Char('s'));
    assert!(app.states_ref().unwrap().skirmish.stress_view);
    assert!(!app.states_ref().unwrap().skirmish.board.auto_supports);
    assert_eq!(
        app.states_ref()
            .unwrap()
            .skirmish
            .board
            .get_focused_cell()
            .get_coords(),
        coords
    );
    let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(160, 50)).unwrap();
    terminal
        .draw(|frame: &mut ratatui::Frame<'_>| -> () {
            frame.render_widget(&mut app, frame.area());
        })
        .unwrap();
    let text: String = terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|cell: &Cell| -> &str { cell.symbol() })
        .collect();
    assert!(text.contains("Auto supports: OFF"));
    assert!(text.contains("Unit tasks"));
    let title: usize = terminal
        .backend()
        .buffer()
        .content
        .windows("Unit tasks".len())
        .position(|cells: &[Cell]| -> bool {
            cells
                .iter()
                .map(|cell: &Cell| -> &str { cell.symbol() })
                .collect::<String>()
                == "Unit tasks"
        })
        .unwrap();
    assert!(title % 160 > 100);
    press(&mut app, KeyCode::Esc);
    assert!(app.states_ref().unwrap().skirmish.task_view);
    press(&mut app, KeyCode::Enter);
    assert!(!app.states_ref().unwrap().skirmish.task_view);
    assert!(
        app.states_ref()
            .unwrap()
            .skirmish
            .board
            .marked_cells
            .marking_cells
    );
    press(&mut app, KeyCode::Right);
    press(&mut app, KeyCode::Char(' '));
    assert!(
        !app.states_ref().unwrap().skirmish.board.cells[5][2]
            .get_ref_option_unit()
            .get_ref_tasks()[0]
            .auto_supports()
    );
}

#[test]
fn moving_choice_rejects_stone_and_never_digs_collapsed_route() -> () {
    let mut app: App = app();
    put_tunnel(&mut app.states_mut().unwrap().skirmish.board, (5, 3), false);
    press(&mut app, KeyCode::Left);
    press(&mut app, KeyCode::Char(' '));
    press(&mut app, KeyCode::Char('t'));
    press(&mut app, KeyCode::Right);
    assert_eq!(app.states_ref().unwrap().skirmish.selected_task, 1);
    press(&mut app, KeyCode::Enter);
    press(&mut app, KeyCode::Right);
    press(&mut app, KeyCode::Right);
    assert_eq!(
        app.states_ref()
            .unwrap()
            .skirmish
            .board
            .get_focused_cell()
            .get_coords(),
        (5, 3)
    );
    press(&mut app, KeyCode::Char(' '));
    let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
    let Tasks::Moving(task) = &game.board.cells[5][2].get_ref_option_unit().get_ref_tasks()[0]
    else {
        panic!("not moving");
    };
    assert!(!task.dig_if_blocked);
    game.board.cells[5][3].set_structure(Structures::CollapsedTunnel(CollapsedTunnel::new()));
    for _ in 0..200 {
        game.tick_update();
    }
    assert_eq!(game.board.cells[5][3].get_structure().get_durability(), 750);
    assert!(game.board.cells[5][2].get_ref_option_unit().is_some());
}
