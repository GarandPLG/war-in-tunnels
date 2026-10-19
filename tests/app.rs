use clap::Parser;
use ratatui::{
  Terminal,
  backend::TestBackend,
  crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
  layout::Rect,
};
use std::sync::mpsc::{Receiver, channel};
use war_in_tunnels::{
  app::{App, GameStates, View, handle_keybindings, threads::AudioCmd},
  cli::Cli,
};

fn app() -> (App, Receiver<AudioCmd>) {
  let args = Cli::try_parse_from(["game", "--disable-audio"]).unwrap();
  let (tx, rx) = channel();
  let mut app = App::new(args, tx);
  app.window_area = Rect::new(0, 0, 120, 40);
  app.states = Some(GameStates::new(&app.args, &app.window_area));
  (app, rx)
}

fn press(app: &mut App, code: KeyCode) {
  handle_keybindings(app, KeyEvent::new(code, KeyModifiers::NONE));
}

#[test]
fn menu_selection_stays_in_bounds_and_opens_each_view() {
  let (mut app, _) = app();
  for _ in 0..10 {
    press(&mut app, KeyCode::Up);
  }
  assert_eq!(app.states_ref().unwrap().main_menu.selected_view, 1);
  for view in [View::Skirmish, View::PerkDecks, View::SkillsConfig] {
    press(&mut app, KeyCode::Char(' '));
    assert_eq!(app.view, view);
    press(&mut app, KeyCode::Esc);
    assert_eq!(app.view, View::MainMenu);
    press(&mut app, KeyCode::Down);
  }
  for _ in 0..10 {
    press(&mut app, KeyCode::Down);
  }
  assert_eq!(app.states_ref().unwrap().main_menu.selected_view, 3);
}

#[test]
fn quit_keys_work_in_every_view() {
  for view in [
    View::MainMenu,
    View::Skirmish,
    View::PerkDecks,
    View::SkillsConfig,
  ] {
    for key in [
      KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE),
      KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL),
    ] {
      let (mut app, _) = app();
      app.view = view;
      handle_keybindings(&mut app, key);
      assert!(app.exit);
    }
  }
}

#[test]
fn key_release_does_not_trigger_quit() {
  let (mut app, _) = app();
  handle_keybindings(
    &mut app,
    KeyEvent::new_with_kind(
      KeyCode::Char('q'),
      KeyModifiers::NONE,
      KeyEventKind::Release,
    ),
  );
  assert!(!app.exit);
}

#[test]
fn music_keys_send_commands_without_opening_audio_device() {
  let (mut app, rx) = app();
  press(&mut app, KeyCode::Char('m'));
  press(&mut app, KeyCode::Char('b'));
  press(&mut app, KeyCode::Char('n'));
  assert!(matches!(rx.try_recv().unwrap(), AudioCmd::Mute));
  assert!(matches!(rx.try_recv().unwrap(), AudioCmd::VolumeUp));
  assert!(matches!(rx.try_recv().unwrap(), AudioCmd::VolumeDown));
  drop(rx);
  press(&mut app, KeyCode::Char('m'));
  assert!(!app.exit);
}

#[test]
fn skirmish_keys_move_focus_toggle_panel_and_limit_zoom() {
  let (mut app, _) = app();
  app.view = View::Skirmish;
  press(&mut app, KeyCode::Right);
  assert_eq!(
    app
      .states_ref()
      .unwrap()
      .skirmish
      .board
      .get_focused_cell()
      .get_coords(),
    (5, 4)
  );
  press(&mut app, KeyCode::Tab);
  assert!(app.states_ref().unwrap().skirmish.side_panel);
  press(&mut app, KeyCode::Tab);
  assert!(!app.states_ref().unwrap().skirmish.side_panel);
  for _ in 0..4 {
    press(&mut app, KeyCode::Char(','));
  }
  assert_eq!(
    app.states_ref().unwrap().skirmish.board.zoom_level,
    war_in_tunnels::app::states::skirmish_states::ZoomLevel::ZoomedIn
  );
  for _ in 0..4 {
    press(&mut app, KeyCode::Char('.'));
  }
  assert_eq!(
    app.states_ref().unwrap().skirmish.board.zoom_level,
    war_in_tunnels::app::states::skirmish_states::ZoomLevel::ZoomedOut
  );
}

#[test]
fn all_views_render_without_real_terminal() {
  let (mut app, _) = app();
  let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
  for view in [
    View::MainMenu,
    View::Skirmish,
    View::PerkDecks,
    View::SkillsConfig,
  ] {
    app.view = view;
    let before = app.states.clone();
    terminal
      .draw(|frame| frame.render_widget(&mut app, frame.area()))
      .unwrap();
    let buffer = terminal.backend().buffer();
    let text: String = buffer.content.iter().map(|cell| cell.symbol()).collect();
    assert!(!text.trim().is_empty(), "{view:?}");
    if matches!(view, View::PerkDecks | View::SkillsConfig) {
      assert!(text.contains("Work in progress"));
    }
    assert_eq!(app.states, before);
  }
}

#[test]
fn common_keys_work_before_states_are_initialized() {
  let (mut app, _) = app();
  app.states = None;
  press(&mut app, KeyCode::Down);
  press(&mut app, KeyCode::Char('q'));
  assert!(app.exit);
}
