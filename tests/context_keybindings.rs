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
        states::skirmish_states::structures::Structures, threads::AudioCmd,
    },
    cli::Cli,
};

fn app() -> App {
    let args: Cli = Cli::try_parse_from(["game", "--disable-audio"]).unwrap();
    let (tx, _rx): (Sender<AudioCmd>, Receiver<AudioCmd>) = channel();
    let mut app: App = App::new(args, tx);
    app.view = View::Skirmish;
    app.window_area = Rect::new(0, 0, 160, 40);
    app.states = Some(GameStates::new(&app.args, &app.window_area));
    app
}

fn press(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> () {
    handle_keybindings(app, KeyEvent::new(code, modifiers));
}

#[test]
fn selecting_miner_switches_help_and_escape_cancels_only_the_route() -> () {
    let mut app: App = app();
    assert!(
        !available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Space)
    );
    press(&mut app, KeyCode::Left, KeyModifiers::NONE);
    assert!(
        available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Space)
    );
    press(&mut app, KeyCode::Char(' '), KeyModifiers::NONE);
    let actions: Vec<Action> = available_skirmish_actions(&app.states_ref().unwrap().skirmish);
    assert!(actions.contains(&Action::Backspace));
    assert!(actions.contains(&Action::Delete));
    assert!(!actions.contains(&Action::Building));
    assert!(!actions.contains(&Action::ZoomIn));
    press(&mut app, KeyCode::Char('b'), KeyModifiers::NONE);
    assert!(!app.states_ref().unwrap().skirmish.building_view);
    press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
    assert_eq!(app.view, View::Skirmish);
    assert!(
        !app.states_ref()
            .unwrap()
            .skirmish
            .board
            .marked_cells
            .marking_cells
    );
    assert!(
        app.states_ref()
            .unwrap()
            .skirmish
            .board
            .get_marked_cells()
            .is_empty()
    );
}

#[test]
fn building_view_has_its_own_help_and_does_not_build_on_invalid_tile() -> () {
    let mut app: App = app();
    let before: war_in_tunnels::app::states::skirmish_states::BoardState =
        app.states_ref().unwrap().skirmish.board.clone();
    press(&mut app, KeyCode::Char('b'), KeyModifiers::NONE);
    assert!(app.states_ref().unwrap().skirmish.building_view);
    assert_eq!(
        available_skirmish_actions(&app.states_ref().unwrap().skirmish),
        [
            Action::Building,
            Action::Left,
            Action::Right,
            Action::Stress,
            Action::Quit2
        ]
    );
    press(&mut app, KeyCode::Left, KeyModifiers::NONE);
    press(&mut app, KeyCode::Right, KeyModifiers::NONE);
    press(&mut app, KeyCode::Enter, KeyModifiers::NONE);
    press(&mut app, KeyCode::Char(' '), KeyModifiers::NONE);
    assert_eq!(app.states_ref().unwrap().skirmish.board, before);
    let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(160, 40)).unwrap();
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
    assert!(text.contains("Construction unavailable"));
    assert!(text.contains("Close buildings"));
    assert!(!text.contains("Marking cells"));
    assert!(!text.contains("Esc -"));
    press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
    assert!(app.states_ref().unwrap().skirmish.building_view);
    assert_eq!(app.view, View::Skirmish);
    press(&mut app, KeyCode::Char('b'), KeyModifiers::NONE);
    assert!(!app.states_ref().unwrap().skirmish.building_view);
    assert_eq!(app.view, View::Skirmish);
    for row in &app.states_ref().unwrap().skirmish.board.cells {
        for cell in row {
            assert!(!matches!(cell.get_structure(), Structures::Mine(_)));
        }
    }
}

#[test]
fn building_key_toggles_panel_and_old_shortcuts_do_not_open_it() -> () {
    let mut app: App = app();
    press(&mut app, KeyCode::Null, KeyModifiers::NONE);
    press(&mut app, KeyCode::Char(' '), KeyModifiers::CONTROL);
    press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
    assert!(!app.states_ref().unwrap().skirmish.building_view);
    press(&mut app, KeyCode::Char('b'), KeyModifiers::NONE);
    assert!(app.states_ref().unwrap().skirmish.building_view);
    press(&mut app, KeyCode::Char('b'), KeyModifiers::NONE);
    assert!(!app.states_ref().unwrap().skirmish.building_view);
    press(&mut app, KeyCode::Char('b'), KeyModifiers::NONE);
    press(&mut app, KeyCode::Char('c'), KeyModifiers::CONTROL);
    assert!(app.exit);
}

#[test]
fn contextual_help_fits_normal_terminal_and_changes_when_route_is_selected() -> () {
    let mut app: App = app();
    app.window_area = Rect::new(0, 0, 120, 40);
    press(&mut app, KeyCode::Left, KeyModifiers::NONE);
    let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(120, 40)).unwrap();
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
    assert!(text.contains("b - Build"));
    assert!(text.contains("Space - Select miner"));
    press(&mut app, KeyCode::Char(' '), KeyModifiers::NONE);
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
    assert!(text.contains("Space - Confirm route"));
    assert!(text.contains("Esc - Cancel route"));
    assert!(!text.contains("b - Build"));
    assert!(!text.contains("Zoom in"));
}

#[test]
fn construction_and_recruitment_popups_render_semantic_text_colors() -> () {
    for (key, name, status, status_color) in [
        ('b', "Mine", "Construction unavailable", Color::Red),
        ('r', "Miner", "Enter: queue Miner", Color::Green),
    ] {
        let mut app: App = app();
        if key == 'r' {
            press(&mut app, KeyCode::Left, KeyModifiers::NONE);
        }
        press(&mut app, KeyCode::Char(key), KeyModifiers::NONE);
        let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(160, 40)).unwrap();
        terminal
            .draw(|frame: &mut ratatui::Frame<'_>| -> () {
                frame.render_widget(&mut app, frame.area());
            })
            .unwrap();
        let cells: &[Cell] = &terminal.backend().buffer().content;
        for (text, color) in [
            (name, Color::LightMagenta),
            ("Cost:", Color::Yellow),
            (status, status_color),
        ] {
            assert!(
                cells.windows(text.len()).any(|window: &[Cell]| -> bool {
                    window
                        .iter()
                        .map(|cell: &Cell| -> &str { cell.symbol() })
                        .collect::<String>()
                        == text
                        && window
                            .iter()
                            .all(|cell: &Cell| -> bool { cell.fg == color })
                }),
                "missing colored text: {text}"
            );
        }
    }
}

#[test]
fn popup_tabs_cycle_without_moving_map_or_changing_queues() -> () {
    for key in ['b', 'r'] {
        let mut app: App = app();
        if key == 'r' {
            press(&mut app, KeyCode::Left, KeyModifiers::NONE);
        }
        let before: war_in_tunnels::app::states::skirmish_states::BoardState =
            app.states_ref().unwrap().skirmish.board.clone();
        press(&mut app, KeyCode::Char(key), KeyModifiers::NONE);
        for (code, index) in [
            (KeyCode::Left, 1),
            (KeyCode::Right, 0),
            (KeyCode::Right, 1),
            (KeyCode::Left, 0),
        ] {
            press(&mut app, code, KeyModifiers::NONE);
            let state: &war_in_tunnels::app::states::SkirmishState =
                &app.states_ref().unwrap().skirmish;
            assert_eq!(state.board, before);
            assert_eq!(state.selected_building, if key == 'b' { index } else { 0 });
            assert_eq!(state.selected_recruit, 0);
            assert!(available_skirmish_actions(state).contains(&Action::Left));
            assert!(available_skirmish_actions(state).contains(&Action::Right));
        }
        press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
        assert_eq!(app.view, View::Skirmish);
        press(&mut app, KeyCode::Char(key), KeyModifiers::NONE);
        assert!(!app.states_ref().unwrap().skirmish.building_view);
        assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
    }
}
