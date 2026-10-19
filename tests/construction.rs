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
        BoardState, MoveFocusedCell, Players, Resources, ZoomLevel,
        resources::MINE_PRODUCTION_TICKS,
        structures::{
          BaseBuilding, BuildingTerrain, Mine, MineConstruction, Ore, Structure, Structures, Tunnel,
        },
        tasks::Tasks,
        units::{MinerUnit, Unit, Units},
      },
    },
    threads::{AudioCmd, TICK_MS},
  },
  cli::Cli,
};

fn board() -> BoardState {
  BoardState::new(&Rect::new(0, 0, 160, 50), 42, 11, ZoomLevel::Default, false)
}

fn complete_mine(board: &mut BoardState, coords: (usize, usize)) -> () {
  assert!(board.start_mine_construction(coords, Players::Player));
  for _ in 0..MineConstruction::BUILD_TICKS {
    board.advance_construction();
  }
  assert!(matches!(
    board.cells[coords.0][coords.1].get_structure(),
    Structures::Mine(_)
  ));
}

fn app() -> App {
  let args: Cli = Cli::try_parse_from(["game", "--disable-audio"]).unwrap();
  let (tx, _rx): (Sender<AudioCmd>, Receiver<AudioCmd>) = channel();
  let mut app: App = App::new(args, tx);
  app.view = View::Skirmish;
  app.window_area = Rect::new(0, 0, 160, 50);
  app.states = Some(GameStates::new(&app.args, &app.window_area));
  app
}

fn travel(board: &mut BoardState, direction: MoveFocusedCell) -> () {
  board.toggle_marking();
  board.change_focused_cell(direction);
  board.toggle_marking();
  for _ in 0..3000_u16.div_ceil(u16::from(TICK_MS)) {
    board.advance_turn();
  }
}

#[test]
fn miner_has_builder_permission_through_all_unit_wrappers() -> () {
  let miner: MinerUnit = MinerUnit::new(Players::Player, (5, 2));
  assert!(miner.get_can_build());
  let unit: Units = Units::Miner(miner);
  assert!(unit.get_can_build());
  assert!(Some(unit).get_can_build());
  let no_unit: Option<Units> = None;
  assert!(!no_unit.get_can_build());
}

#[test]
fn mine_requires_a_cardinal_friendly_builder_and_preserves_target_on_failure() -> () {
  let mut board: BoardState = board();
  let original: Structures = board.cells[5][1].get_structure();
  board.cells[5][2].get_mut_option_unit().take();
  assert!(!board.start_mine_construction((5, 1), Players::Player));
  *board.cells[5][2].get_mut_option_unit() =
    Some(Units::Miner(MinerUnit::new(Players::Enemy, (5, 2))));
  assert!(!board.start_mine_construction((5, 1), Players::Player));
  board.cells[5][2].get_mut_option_unit().take();
  *board.cells[4][2].get_mut_option_unit() =
    Some(Units::Miner(MinerUnit::new(Players::Player, (4, 2))));
  assert!(!board.start_mine_construction((5, 1), Players::Player));
  assert_eq!(board.cells[5][1].get_structure(), original);
  assert!(!board.start_mine_construction((usize::MAX, 1), Players::Player));
  assert!(!board.start_mine_construction((5, 1), Players::Unclaimed));
}

#[test]
fn ore_exception_does_not_require_a_base_or_tunnel_connection() -> () {
  let mut board: BoardState = board();
  board.cells[0][0].set_structure(Structures::Ore(Ore::new(Players::Unclaimed, 1)));
  *board.cells[0][1].get_mut_option_unit() =
    Some(Units::Miner(MinerUnit::new(Players::Player, (0, 1))));
  assert!(board.can_build_at((0, 0), Players::Player, BuildingTerrain::Ore));
  complete_mine(&mut board, (0, 0));
  assert_eq!(
    board.cells[0][0].get_structure().get_owner(),
    Players::Player
  );
  assert!(!board.start_mine_construction((0, 0), Players::Player));
}

#[test]
fn ordinary_building_sites_require_stone_connected_to_base_or_tunnel() -> () {
  let mut board: BoardState = board();
  assert!(board.can_build_at((5, 3), Players::Player, BuildingTerrain::Stone));
  assert!(!board.can_build_at((5, 3), Players::Player, BuildingTerrain::Ore));
  *board.cells[5][5].get_mut_option_unit() =
    Some(Units::Miner(MinerUnit::new(Players::Player, (5, 5))));
  assert!(!board.can_build_at((5, 4), Players::Player, BuildingTerrain::Stone));
  board.cells[5][5].set_structure(Structures::Tunnel(Tunnel::new()));
  assert!(board.can_build_at((5, 4), Players::Player, BuildingTerrain::Stone));
  board.cells[5][5].set_structure(Structures::Base(BaseBuilding::new(Players::Enemy)));
  assert!(!board.can_build_at((5, 4), Players::Player, BuildingTerrain::Stone));
  board.cells[5][3].set_structure(Structures::Tunnel(Tunnel::new()));
  assert!(!board.can_build_at((5, 3), Players::Player, BuildingTerrain::Stone));
}

#[test]
fn occupied_depleted_or_claimed_deposit_cannot_be_built_on() -> () {
  let mut board: BoardState = board();
  *board.cells[5][1].get_mut_option_unit() =
    Some(Units::Miner(MinerUnit::new(Players::Player, (5, 1))));
  assert!(!board.start_mine_construction((5, 1), Players::Player));
  board.cells[5][1].get_mut_option_unit().take();
  let mut ore: Ore = Ore::new(Players::Unclaimed, 1);
  ore.extract(5000);
  board.cells[5][1].set_structure(Structures::Ore(ore));
  assert!(!board.start_mine_construction((5, 1), Players::Player));
  board.cells[5][1].set_structure(Structures::Ore(Ore::new(Players::Enemy, 1)));
  assert!(!board.start_mine_construction((5, 1), Players::Player));
}

#[test]
fn construction_view_starts_paid_build_on_enter_and_stays_open() -> () {
  let mut app: App = app();
  for _ in 0..2 {
    handle_keybindings(&mut app, KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
  }
  handle_keybindings(
    &mut app,
    KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL),
  );
  assert!(available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Enter));
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
  assert!(text.contains("Ready to build: Enter"));
  assert!(text.contains("Enter - Build Mine"));
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
  let state: &SkirmishState = &app.states_ref().unwrap().skirmish;
  assert!(state.building_view);
  assert!(matches!(
    state.board.cells[5][1].get_structure(),
    Structures::MineConstruction(_)
  ));
  assert_eq!(state.player_resources.wood, 30);
  assert_eq!(state.player_resources.metal, 15);
}

#[test]
fn miner_can_travel_into_and_out_of_friendly_mine_with_moving_tasks() -> () {
  let mut board: BoardState = board();
  complete_mine(&mut board, (5, 1));
  board.change_focused_cell(MoveFocusedCell::Left);
  board.toggle_marking();
  board.change_focused_cell(MoveFocusedCell::Left);
  board.toggle_marking();
  let unit: &Units = board.cells[5][2].get_ref_option_unit().as_ref().unwrap();
  assert!(matches!(
    unit.get_ref_tasks().front(),
    Some(Tasks::Moving(_))
  ));
  for _ in 1..3000_u16.div_ceil(u16::from(TICK_MS)) {
    board.advance_turn();
  }
  assert!(board.cells[5][1].get_ref_option_unit().is_none());
  board.advance_turn();
  assert_eq!(board.cells[5][1].get_ref_option_unit().get_coords(), (5, 1));
  assert!(board.cells[5][2].get_ref_option_unit().is_none());
  travel(&mut board, MoveFocusedCell::Right);
  assert!(board.cells[5][1].get_ref_option_unit().is_none());
  assert_eq!(board.cells[5][2].get_ref_option_unit().get_coords(), (5, 2));
}

#[test]
fn staffed_mine_doubles_production_and_bonus_stops_after_departure() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  complete_mine(&mut game.board, (5, 1));
  game.turn_counter = MINE_PRODUCTION_TICKS - 1;
  game.tick_update();
  assert_eq!(game.player_resources.metal, 26);
  game.board.change_focused_cell(MoveFocusedCell::Left);
  travel(&mut game.board, MoveFocusedCell::Left);
  game.turn_counter = MINE_PRODUCTION_TICKS - 1;
  game.tick_update();
  assert_eq!(game.player_resources.metal, 28);
  travel(&mut game.board, MoveFocusedCell::Right);
  game.turn_counter = MINE_PRODUCTION_TICKS - 1;
  game.tick_update();
  assert_eq!(game.player_resources.metal, 29);
  let Structures::Mine(mine) = game.board.cells[5][1].get_structure() else {
    panic!("mine missing");
  };
  assert_eq!(mine.get_ore_amount(), 4996);
}

#[test]
fn mine_rejects_enemy_miners_and_staffing_does_not_overdraw_ore() -> () {
  let mut ore: Ore = Ore::new(Players::Unclaimed, 1);
  ore.extract(4999);
  let mut mine: Mine = Mine::new(Players::Player, ore).unwrap();
  let player: Units = Units::Miner(MinerUnit::new(Players::Player, (5, 2)));
  let enemy: Units = Units::Miner(MinerUnit::new(Players::Enemy, (5, 2)));
  assert!(mine.accepts_unit(&player));
  assert!(!mine.accepts_unit(&enemy));
  assert!(!Structures::Mine(mine).can_enter(&enemy));
  assert_eq!(mine.produce_with_miner(true), 1);
  assert_eq!(mine.produce_with_miner(true), 0);
}

#[test]
fn remote_ore_cannot_start_construction_or_consume_resources() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  let ore: Structures = Structures::Ore(Ore::new(Players::Unclaimed, 1));
  game.board.cells[5][21].set_structure(ore);
  let before: Resources = game.player_resources;
  assert!(!game.start_mine_construction((5, 21), Players::Player));
  assert_eq!(game.player_resources, before);
  assert_eq!(game.board.cells[5][21].get_structure(), ore);
}

#[test]
fn construction_payment_is_atomic_and_exact_funds_are_enough() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  for resources in [
    Resources {
      wood: 19,
      metal: 10,
    },
    Resources { wood: 20, metal: 9 },
  ] {
    game.player_resources = resources;
    assert!(!game.can_start_mine_construction((5, 1), Players::Player));
    assert!(!game.start_mine_construction((5, 1), Players::Player));
    assert_eq!(game.player_resources, resources);
    assert!(matches!(
      game.board.cells[5][1].get_structure(),
      Structures::Ore(_)
    ));
  }
  game.player_resources = Mine::COST;
  assert!(game.start_mine_construction((5, 1), Players::Player));
  assert_eq!(game.player_resources, Resources { wood: 0, metal: 0 });
  assert!(!game.start_mine_construction((5, 1), Players::Player));
}

#[test]
fn construction_takes_ten_seconds_and_cannot_produce_or_accept_miners_early() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  assert!(game.start_mine_construction((5, 1), Players::Player));
  for _ in 1..MineConstruction::BUILD_TICKS {
    game.tick_update();
  }
  let structure: Structures = game.board.cells[5][1].get_structure();
  let Structures::MineConstruction(site) = structure else {
    panic!("completed too early");
  };
  assert_eq!(site.progress_percent(), 99);
  assert!(site.into_mine().is_none());
  assert_eq!(site.cancel().get_ore_amount(), 5000);
  assert!(
    !structure.can_enter(
      game.board.cells[5][2]
        .get_ref_option_unit()
        .as_ref()
        .unwrap()
    )
  );
  assert_eq!(game.player_resources.metal, 15);
  game.tick_update();
  assert!(matches!(
    game.board.cells[5][1].get_structure(),
    Structures::Mine(_)
  ));
  assert_eq!(game.player_resources.metal, 15);
}

#[test]
fn construction_pauses_without_nearby_builder_and_resumes_after_return() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  assert!(game.start_mine_construction((5, 1), Players::Player));
  for _ in 0..100 {
    game.tick_update();
  }
  let before: Structures = game.board.cells[5][1].get_structure();
  let builder: Option<Units> = game.board.cells[5][2].get_mut_option_unit().take();
  for _ in 0..MineConstruction::BUILD_TICKS {
    game.tick_update();
  }
  assert_eq!(game.board.cells[5][1].get_structure(), before);
  assert_eq!(game.player_resources.metal, 15);
  *game.board.cells[5][2].get_mut_option_unit() = builder;
  for _ in 100..MineConstruction::BUILD_TICKS {
    game.tick_update();
  }
  assert!(matches!(
    game.board.cells[5][1].get_structure(),
    Structures::Mine(_)
  ));
}

#[test]
fn cancellation_restores_original_ore_and_refunds_cost_exactly_once() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  let mut ore: Ore = Ore::new(Players::Unclaimed, 1);
  ore.extract(123);
  game.board.cells[5][1].set_structure(Structures::Ore(ore));
  let before: Resources = game.player_resources;
  assert!(game.start_mine_construction((5, 1), Players::Player));
  for _ in 0..100 {
    game.tick_update();
  }
  game.board.cells[5][2].get_mut_option_unit().take();
  assert!(game.cancel_mine_construction((5, 1), Players::Player));
  assert_eq!(game.player_resources, before);
  assert_eq!(game.board.cells[5][1].get_structure(), Structures::Ore(ore));
  assert!(!game.cancel_mine_construction((5, 1), Players::Player));
  assert_eq!(game.player_resources, before);
}

#[test]
fn panel_shows_progress_and_delete_cancels_while_escape_only_closes_panel() -> () {
  let mut app: App = app();
  for _ in 0..2 {
    handle_keybindings(&mut app, KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
  }
  handle_keybindings(
    &mut app,
    KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL),
  );
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
  assert!(
    available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Delete)
  );
  assert!(
    !available_skirmish_actions(&app.states_ref().unwrap().skirmish).contains(&Action::Enter)
  );
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
  assert_eq!(app.states_ref().unwrap().skirmish.player_resources.wood, 30);
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
  assert!(text.contains("Cost: 20 Wood + 10 Metal"));
  assert!(text.contains("Construction: 0%"));
  assert!(text.contains("Delete - Cancel construction"));
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
  assert!(matches!(
    app.states_ref().unwrap().skirmish.board.cells[5][1].get_structure(),
    Structures::MineConstruction(_)
  ));
  handle_keybindings(
    &mut app,
    KeyEvent::new(KeyCode::Char(' '), KeyModifiers::CONTROL),
  );
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Delete, KeyModifiers::NONE));
  assert!(matches!(
    app.states_ref().unwrap().skirmish.board.cells[5][1].get_structure(),
    Structures::Ore(_)
  ));
  assert_eq!(
    app.states_ref().unwrap().skirmish.player_resources,
    Resources::new(50, 25)
  );
}

#[test]
fn player_cannot_cancel_enemy_site_or_refund_a_completed_mine() -> () {
  let mut app: App = app();
  let game: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  assert!(game.start_mine_construction((5, 40), Players::Enemy));
  assert!(!game.cancel_mine_construction((5, 40), Players::Player));
  assert_eq!(game.player_resources, Resources::new(50, 25));
  for _ in 0..MineConstruction::BUILD_TICKS {
    game.tick_update();
  }
  assert!(!game.cancel_mine_construction((5, 40), Players::Enemy));
  assert_eq!(game.enemy_resources.metal, 15);
}
