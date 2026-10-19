use clap::Parser;
use ratatui::{
  Terminal,
  backend::TestBackend,
  buffer::Cell,
  crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
  layout::Rect,
};
use std::sync::mpsc::{Receiver, Sender, channel};
use war_in_tunnels::{
  app::{
    Action, App, GameStates, View, available_skirmish_actions, handle_keybindings,
    states::{
      SkirmishState,
      skirmish_states::{
        MoveFocusedCell, Players, Resources,
        structures::{BaseBuilding, Structures, Tunnel},
        units::{MinerUnit, Unit, Units},
      },
    },
    threads::{AudioCmd, TICK_MS},
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

fn base(game: &SkirmishState, coords: (usize, usize)) -> BaseBuilding {
  let Structures::Base(base) = game.board.cells[coords.0][coords.1].get_structure() else {
    panic!("base missing");
  };
  base
}

fn press(app: &mut App, code: KeyCode, modifiers: KeyModifiers) -> () {
  handle_keybindings(app, KeyEvent::new(code, modifiers));
}

#[test]
fn recruitment_is_owned_by_base_and_invalid_requests_do_not_charge() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  let before: Resources = game.player_resources;
  for coords in [(5, 39), (0, 0), (usize::MAX, 2)] {
    assert!(!game.recruit_miner(coords, Players::Player));
    assert_eq!(game.player_resources, before);
  }
  assert!(!game.recruit_miner((5, 2), Players::Unclaimed));
  assert!(game.recruit_miner((5, 2), Players::Player));
  assert_eq!(game.player_resources, Resources::new(40, 10));
  assert_eq!(game.enemy_resources, Resources::new(50, 25));
  assert_eq!(base(game, (5, 2)).queued_miners(), 1);
}

#[test]
fn recruitment_requires_both_resources_and_charges_each_queue_entry() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  for resources in [
    Resources { wood: 9, metal: 15 },
    Resources {
      wood: 10,
      metal: 14,
    },
  ] {
    game.player_resources = resources;
    assert!(!game.can_recruit_miner((5, 2), Players::Player));
    assert!(!game.recruit_miner((5, 2), Players::Player));
    assert_eq!(game.player_resources, resources);
    assert_eq!(base(game, (5, 2)).queued_miners(), 0);
  }
  game.player_resources = Resources::new(20, 30);
  assert!(game.recruit_miner((5, 2), Players::Player));
  assert!(game.recruit_miner((5, 2), Players::Player));
  assert_eq!(base(game, (5, 2)).queued_miners(), 2);
  assert_eq!(game.player_resources, Resources::new(0, 0));
  assert!(!game.recruit_miner((5, 2), Players::Player));
}

#[test]
fn first_recruit_waits_at_full_progress_and_following_recruits_pause_until_departure() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  game.player_resources = Resources::new(100, 100);
  assert!(game.recruit_miner((5, 2), Players::Player));
  assert!(game.recruit_miner((5, 2), Players::Player));
  let original: Option<Units> = game.board.cells[5][2].get_option_unit();
  for _ in 0..2 * BaseBuilding::RECRUITMENT_TICKS {
    game.tick_update();
  }
  assert_eq!(base(game, (5, 2)).queued_miners(), 2);
  assert_eq!(base(game, (5, 2)).recruitment_progress(), 100);
  assert_eq!(game.board.cells[5][2].get_option_unit(), original);
  game.board.cells[5][2].get_mut_option_unit().take();
  game.tick_update();
  let spawned: Option<Units> = game.board.cells[5][2].get_option_unit();
  assert_eq!(spawned.get_owner(), Players::Player);
  assert_eq!(spawned.get_coords(), (5, 2));
  assert!(spawned.get_ref_tasks().is_empty());
  assert_eq!(base(game, (5, 2)).queued_miners(), 1);
  for _ in 0..2 * BaseBuilding::RECRUITMENT_TICKS {
    game.tick_update();
  }
  assert_eq!(base(game, (5, 2)).queued_miners(), 1);
  assert_eq!(base(game, (5, 2)).recruitment_progress(), 0);
  assert_eq!(game.board.cells[5][2].get_option_unit(), spawned);
  game.board.cells[5][2].get_mut_option_unit().take();
  for _ in 1..BaseBuilding::RECRUITMENT_TICKS {
    game.tick_update();
  }
  assert!(game.board.cells[5][2].get_ref_option_unit().is_none());
  assert_eq!(base(game, (5, 2)).recruitment_progress(), 99);
  game.tick_update();
  assert_eq!(base(game, (5, 2)).queued_miners(), 0);
  assert_eq!(
    game.board.cells[5][2].get_ref_option_unit().get_owner(),
    Players::Player
  );
}

#[test]
fn walking_out_of_base_resumes_recruitment_and_new_miner_can_receive_tasks() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  assert!(game.recruit_miner((5, 2), Players::Player));
  game.board.cells[5][3].set_structure(Structures::Tunnel(Tunnel::new()));
  game.board.change_focused_cell(MoveFocusedCell::Left);
  game.board.toggle_marking();
  game.board.change_focused_cell(MoveFocusedCell::Right);
  game.board.toggle_marking();
  for _ in 0..3000_u16.div_ceil(u16::from(TICK_MS)) {
    game.tick_update();
  }
  assert!(game.board.cells[5][2].get_ref_option_unit().is_none());
  for _ in 3000_u16.div_ceil(u16::from(TICK_MS))..BaseBuilding::RECRUITMENT_TICKS - 1 {
    game.tick_update();
  }
  assert!(game.board.cells[5][2].get_ref_option_unit().is_none());
  assert_eq!(base(game, (5, 2)).recruitment_progress(), 99);
  game.tick_update();
  assert_eq!(base(game, (5, 2)).queued_miners(), 0);
  assert_eq!(
    game.board.cells[5][2].get_ref_option_unit().get_coords(),
    (5, 2)
  );
  assert_eq!(
    game.board.cells[5][3].get_ref_option_unit().get_coords(),
    (5, 3)
  );
  game.board.change_focused_cell(MoveFocusedCell::Left);
  game.board.toggle_marking();
  game.board.change_focused_cell(MoveFocusedCell::Down);
  game.board.toggle_marking();
  assert_eq!(
    game.board.cells[5][2]
      .get_ref_option_unit()
      .get_ref_tasks()
      .len(),
    1
  );
}

#[test]
fn arrival_during_training_does_not_overwrite_visitor_and_ready_recruit_spawns_after_departure()
-> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  game.board.cells[5][2].get_mut_option_unit().take();
  assert!(game.recruit_miner((5, 2), Players::Player));
  for _ in 0..100 {
    game.tick_update();
  }
  let visitor: Units = Units::Miner(MinerUnit::new(Players::Enemy, (5, 2)));
  *game.board.cells[5][2].get_mut_option_unit() = Some(visitor.clone());
  for _ in 0..BaseBuilding::RECRUITMENT_TICKS {
    game.tick_update();
  }
  assert_eq!(base(game, (5, 2)).recruitment_progress(), 100);
  assert_eq!(game.board.cells[5][2].get_option_unit(), Some(visitor));
  game.board.cells[5][2].get_mut_option_unit().take();
  game.tick_update();
  assert_eq!(base(game, (5, 2)).queued_miners(), 0);
  assert_eq!(
    game.board.cells[5][2].get_ref_option_unit().get_owner(),
    Players::Player
  );
}

#[test]
fn bases_have_independent_queues_costs_and_spawn_ownership() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  for (coords, owner) in [((5, 2), Players::Player), ((5, 39), Players::Enemy)] {
    game.board.cells[coords.0][coords.1]
      .get_mut_option_unit()
      .take();
    assert!(game.recruit_miner(coords, owner));
  }
  assert_eq!(game.player_resources.metal, 10);
  assert_eq!(game.enemy_resources.metal, 10);
  for _ in 0..BaseBuilding::RECRUITMENT_TICKS {
    game.tick_update();
  }
  for (coords, owner) in [((5, 2), Players::Player), ((5, 39), Players::Enemy)] {
    assert_eq!(base(game, coords).queued_miners(), 0);
    assert_eq!(
      game.board.cells[coords.0][coords.1]
        .get_ref_option_unit()
        .get_owner(),
      owner
    );
    assert_eq!(
      game.board.cells[coords.0][coords.1]
        .get_ref_option_unit()
        .get_coords(),
      coords
    );
  }
}

#[test]
fn recruitment_panel_queues_with_enter_shows_ready_state_and_closes_without_losing_queue() -> () {
  let mut app: App = app();
  press(&mut app, KeyCode::Left, KeyModifiers::NONE);
  assert!(
    available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Recruitment)
  );
  press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
  assert!(app.states_ref().unwrap().skirmish.recruitment_view);
  press(&mut app, KeyCode::Enter, KeyModifiers::NONE);
  press(&mut app, KeyCode::Enter, KeyModifiers::NONE);
  assert_eq!(
    base(&app.states_ref().unwrap().skirmish, (5, 2)).queued_miners(),
    1
  );
  assert!(
    !available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Enter)
  );
  press(&mut app, KeyCode::Right, KeyModifiers::NONE);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::CONTROL);
  assert_eq!(
    app
      .states_ref()
      .unwrap()
      .skirmish
      .board
      .get_focused_cell()
      .get_coords(),
    (5, 2)
  );
  assert!(!app.states_ref().unwrap().skirmish.building_view);
  for _ in 0..BaseBuilding::RECRUITMENT_TICKS {
    app.states_mut().unwrap().skirmish.tick_update();
  }
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
  assert!(text.contains("Cost: 10 Wood + 15 Metal"));
  assert!(text.contains("Queued Miners: 1"));
  assert!(text.contains("Current progress: 100%"));
  assert!(text.contains("Ready: waiting for free base cell"));
  assert!(text.contains("Close recruitment"));
  press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  assert_eq!(app.view, View::Skirmish);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  assert_eq!(
    base(&app.states_ref().unwrap().skirmish, (5, 2)).queued_miners(),
    1
  );
}

#[test]
fn recruitment_panel_is_unavailable_off_base_and_during_route_selection() -> () {
  let mut app: App = app();
  press(&mut app, KeyCode::Char('r'), KeyModifiers::NONE);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  press(&mut app, KeyCode::Left, KeyModifiers::NONE);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::NONE);
  press(&mut app, KeyCode::Char('r'), KeyModifiers::NONE);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  assert!(
    app
      .states_ref()
      .unwrap()
      .skirmish
      .board
      .marked_cells
      .marking_cells
  );
  press(&mut app, KeyCode::Esc, KeyModifiers::NONE);
  press(&mut app, KeyCode::Char(' '), KeyModifiers::ALT);
  press(&mut app, KeyCode::Char('c'), KeyModifiers::CONTROL);
  assert!(app.exit);
}

#[test]
fn recruitment_fallback_opens_queues_and_closes_on_own_base() -> () {
  let mut app: App = app();
  press(&mut app, KeyCode::Left, KeyModifiers::NONE);
  press(&mut app, KeyCode::Char('r'), KeyModifiers::NONE);
  assert!(app.states_ref().unwrap().skirmish.recruitment_view);
  press(&mut app, KeyCode::Enter, KeyModifiers::NONE);
  press(&mut app, KeyCode::Char('r'), KeyModifiers::NONE);
  assert!(!app.states_ref().unwrap().skirmish.recruitment_view);
  assert_eq!(
    base(&app.states_ref().unwrap().skirmish, (5, 2)).queued_miners(),
    1
  );
  assert_eq!(
    app.states_ref().unwrap().skirmish.player_resources,
    Resources::new(40, 10)
  );
}
