use clap::Parser;
use ratatui::{
  Terminal,
  backend::TestBackend,
  buffer::Buffer,
  crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
  layout::Rect,
  widgets::Widget,
};
use std::sync::mpsc::{Receiver, Sender, channel};
use war_in_tunnels::{
  app::{
    App, GameStates, View, handle_keybindings,
    states::{
      SkirmishState,
      skirmish_states::{
        BoardState, CellSizes, MoveFocusedCell, Players, ZoomLevel,
        structures::{Structure, Structures, Tunnel},
        tasks::{DiggingTask, MovingTask, Task, Tasks},
        units::{MinerUnit, Unit, Units},
      },
    },
    threads::{AudioCmd, TICK_MS},
  },
  cli::Cli,
};

fn game() -> SkirmishState {
  let args = Cli::try_parse_from(["game", "--disable-audio"]).unwrap();
  GameStates::new(&args, &Rect::new(0, 0, 120, 40)).skirmish
}

fn queue(board: &mut BoardState, origin: (usize, usize), path: &[(usize, usize)]) {
  let unit = board.cells[origin.0][origin.1]
    .get_mut_option_unit()
    .as_mut()
    .unwrap();
  let power = unit.get_digging_power();
  unit.set_task(Tasks::Digging(DiggingTask::new(
    path.iter().copied().collect(),
    power,
  )));
}

fn tunnel(board: &mut BoardState, row: usize, col: usize) {
  board.cells[row][col].set_structure(Structures::Tunnel(Tunnel::new()));
}

fn unit(board: &BoardState, row: usize, col: usize) -> &Units {
  board.cells[row][col]
    .get_ref_option_unit()
    .as_ref()
    .unwrap()
}

fn travel_ticks() -> u16 {
  3000_u16.div_ceil(u16::from(TICK_MS))
}

fn advance(game: &mut SkirmishState, ticks: u16) {
  for _ in 0..ticks {
    game.tick_update();
  }
}

#[test]
fn tick_damages_stone_before_moving_and_finishes_entire_route() {
  let mut game = game();
  queue(&mut game.board, (5, 2), &[(5, 3), (5, 4)]);
  game.tick_update();
  assert_eq!(game.turn_counter, 1);
  let durability = game.board.cells[5][3].get_structure().get_durability();
  assert!(durability < 1000 && durability > 0);
  assert_eq!(unit(&game.board, 5, 2).get_coords(), (5, 2));
  assert!(game.board.cells[5][3].get_ref_option_unit().is_none());
  for _ in 0..500 {
    game.tick_update();
  }
  for col in [3, 4] {
    assert!(matches!(
      game.board.cells[5][col].get_structure(),
      Structures::Tunnel(_)
    ));
  }
  assert!(matches!(
    game.board.cells[5][2].get_structure(),
    Structures::Base(_)
  ));
  assert_eq!(unit(&game.board, 5, 4).get_coords(), (5, 4));
  assert!(unit(&game.board, 5, 4).get_ref_tasks().is_empty());
  assert!(game.board.cells[5][2].get_ref_option_unit().is_none());
  assert!(game.board.cells[5][3].get_ref_option_unit().is_none());
}

#[test]
fn each_tunnel_step_takes_three_seconds_in_both_directions() {
  for path in [[(5, 3), (5, 4)], [(4, 2), (3, 2)]] {
    let mut game = game();
    for (row, col) in path {
      tunnel(&mut game.board, row, col);
    }
    queue(&mut game.board, (5, 2), &path);
    advance(&mut game, travel_ticks() - 1);
    assert_eq!(unit(&game.board, 5, 2).get_coords(), (5, 2));
    game.tick_update();
    assert_eq!(
      unit(&game.board, path[0].0, path[0].1).get_coords(),
      path[0]
    );
    assert_eq!(
      unit(&game.board, path[0].0, path[0].1).get_ref_tasks()[0]
        .get_path()
        .len(),
      1
    );
    advance(&mut game, travel_ticks() - 1);
    assert_eq!(
      unit(&game.board, path[0].0, path[0].1).get_coords(),
      path[0]
    );
    game.tick_update();
    assert!(
      unit(&game.board, path[1].0, path[1].1)
        .get_ref_tasks()
        .is_empty()
    );
  }
}

#[test]
fn queued_tasks_execute_in_order_and_keep_unit_coordinates_current() {
  let mut game = game();
  tunnel(&mut game.board, 5, 3);
  tunnel(&mut game.board, 5, 4);
  queue(&mut game.board, (5, 2), &[(5, 3)]);
  queue(&mut game.board, (5, 2), &[(5, 4)]);
  advance(&mut game, travel_ticks());
  assert_eq!(unit(&game.board, 5, 3).get_ref_tasks().len(), 1);
  advance(&mut game, travel_ticks());
  assert_eq!(unit(&game.board, 5, 4).get_coords(), (5, 4));
  assert!(unit(&game.board, 5, 4).get_ref_tasks().is_empty());
  queue(&mut game.board, (5, 4), &[(5, 3), (5, 2)]);
  advance(&mut game, 2 * travel_ticks() - 1);
  assert_eq!(unit(&game.board, 5, 3).get_coords(), (5, 3));
  game.tick_update();
  assert_eq!(unit(&game.board, 5, 2).get_coords(), (5, 2));
  assert!(matches!(
    game.board.cells[5][2].get_structure(),
    Structures::Base(_)
  ));
}

#[test]
fn occupied_tunnel_waits_without_losing_units_or_task() {
  let mut game = game();
  tunnel(&mut game.board, 5, 3);
  *game.board.cells[5][3].get_mut_option_unit() =
    Some(Units::Miner(MinerUnit::new(Players::Player, (5, 3))));
  queue(&mut game.board, (5, 2), &[(5, 3)]);
  for _ in 0..3 {
    game.tick_update();
  }
  assert_eq!(unit(&game.board, 5, 2).get_ref_tasks().len(), 1);
  assert_eq!(unit(&game.board, 5, 3).get_coords(), (5, 3));
  game.board.cells[5][3].get_mut_option_unit().take();
  advance(&mut game, travel_ticks() - 1);
  assert_eq!(unit(&game.board, 5, 2).get_coords(), (5, 2));
  game.tick_update();
  assert_eq!(unit(&game.board, 5, 3).get_coords(), (5, 3));
  assert!(unit(&game.board, 5, 3).get_ref_tasks().is_empty());
}

#[test]
fn ore_is_not_destroyed_by_digging_task() {
  let mut game = game();
  let ore = game.board.cells[5][1].get_structure();
  queue(&mut game.board, (5, 2), &[(5, 1)]);
  for _ in 0..200 {
    game.tick_update();
  }
  assert_eq!(game.board.cells[5][1].get_structure(), ore);
  assert_eq!(unit(&game.board, 5, 2).get_coords(), (5, 2));
}

#[test]
fn invalid_or_empty_routes_do_not_teleport_or_remove_miner() {
  for path in [vec![], vec![(usize::MAX, usize::MAX)], vec![(0, 0)]] {
    let mut game = game();
    queue(&mut game.board, (5, 2), &path);
    game.tick_update();
    assert_eq!(unit(&game.board, 5, 2).get_coords(), (5, 2));
    assert!(unit(&game.board, 5, 2).get_ref_tasks().is_empty());
  }
}

#[test]
fn only_player_miner_can_start_drawing_route() {
  let mut game = game();
  game.board.toggle_marking();
  assert!(!game.board.marked_cells.marking_cells);
  for _ in 0..36 {
    game.board.change_focused_cell(MoveFocusedCell::Right);
  }
  assert_eq!(game.board.get_focused_cell().get_coords(), (5, 39));
  game.board.toggle_marking();
  assert!(!game.board.marked_cells.marking_cells);
  assert!(game.board.get_marked_cells().is_empty());
}

#[test]
fn drawing_route_pauses_selected_miner_and_boundary_moves_do_not_repeat_cells() {
  let mut game = game();
  game.board.change_focused_cell(MoveFocusedCell::Left);
  queue(&mut game.board, (5, 2), &[(5, 3)]);
  game.board.toggle_marking();
  game.tick_update();
  assert_eq!(
    game.board.cells[5][3].get_structure().get_durability(),
    1000
  );
  for _ in 0..10 {
    game.board.change_focused_cell(MoveFocusedCell::Up);
  }
  let coords: Vec<_> = game.board.get_marked_cells().iter().copied().collect();
  assert!(coords.windows(2).all(|pair| pair[0] != pair[1]));
  game.board.end_marking_cells(true);
  game.tick_update();
  assert!(game.board.cells[5][3].get_structure().get_durability() < 1000);
}

#[test]
fn drawing_another_task_extends_the_end_of_queued_work() {
  let mut game = game();
  for col in [3, 4, 5] {
    tunnel(&mut game.board, 5, col);
  }
  game.board.change_focused_cell(MoveFocusedCell::Left);
  game.board.toggle_marking();
  game.board.change_focused_cell(MoveFocusedCell::Right);
  game.board.change_focused_cell(MoveFocusedCell::Right);
  game.board.toggle_marking();
  game.board.change_focused_cell(MoveFocusedCell::Left);
  game.board.change_focused_cell(MoveFocusedCell::Left);
  game.board.toggle_marking();
  assert_eq!(game.board.get_focused_cell().get_coords(), (5, 4));
  game.board.change_focused_cell(MoveFocusedCell::Right);
  game.board.toggle_marking();
  assert_eq!(unit(&game.board, 5, 2).get_ref_tasks().len(), 2);
  advance(&mut game, 3 * travel_ticks());
  assert_eq!(unit(&game.board, 5, 5).get_coords(), (5, 5));
  assert!(unit(&game.board, 5, 5).get_ref_tasks().is_empty());
}

#[test]
fn freshly_dug_cell_is_entered_immediately_then_tunnel_travel_takes_three_seconds() {
  let mut game = game();
  tunnel(&mut game.board, 5, 4);
  queue(&mut game.board, (5, 2), &[(5, 3), (5, 4)]);
  for _ in 0..200 {
    game.tick_update();
    if matches!(
      game.board.cells[5][3].get_structure(),
      Structures::Tunnel(_)
    ) {
      assert_eq!(unit(&game.board, 5, 3).get_coords(), (5, 3));
      advance(&mut game, travel_ticks() - 1);
      assert_eq!(unit(&game.board, 5, 3).get_coords(), (5, 3));
      game.tick_update();
      assert!(unit(&game.board, 5, 4).get_ref_tasks().is_empty());
      return;
    }
  }
  panic!("miner did not finish digging the stone");
}

#[test]
fn miner_symbol_in_tunnel_uses_owner_color_even_when_selected_or_marked() {
  for owner in [Players::Player, Players::Enemy] {
    for zoom in [ZoomLevel::Default, ZoomLevel::ZoomedIn] {
      for (selected, marked) in [(false, false), (true, false), (false, true), (true, true)] {
        let mut game = game();
        let cell = &mut game.board.cells[5][3];
        cell.set_structure(Structures::Tunnel(Tunnel::new()));
        cell
          .set_selected(selected)
          .set_marked(marked)
          .set_zoom_level(zoom);
        *cell.get_mut_option_unit() = Some(Units::Miner(MinerUnit::new(owner, (5, 3))));
        let area = Rect::new(
          0,
          0,
          zoom.get_cell_size(CellSizes::Width) as u16,
          zoom.get_cell_size(CellSizes::Height) as u16,
        );
        let mut buffer = Buffer::empty(area);
        cell.clone().render(area, &mut buffer);
        let marker = buffer
          .content
          .iter()
          .find(|cell| cell.symbol() == "M")
          .unwrap();
        assert_eq!(marker.fg, owner.get_color(), "{owner:?} {zoom:?}");
        assert!(buffer.content.iter().any(|cell| cell.symbol() == "T"));
      }
    }
  }
}

#[test]
fn tui_shows_movement_percentage_and_clears_it_after_each_step() {
  for zoom in [
    ZoomLevel::ZoomedIn,
    ZoomLevel::Default,
    ZoomLevel::ZoomedOut,
  ] {
    let mut game = game();
    for col in [2, 3, 4] {
      tunnel(&mut game.board, 5, col);
    }
    queue(&mut game.board, (5, 2), &[(5, 3), (5, 4)]);
    let render = |board: &BoardState, col: usize| {
      let area = Rect::new(
        0,
        0,
        zoom.get_cell_size(CellSizes::Width) as u16,
        zoom.get_cell_size(CellSizes::Height) as u16,
      );
      let mut buffer = Buffer::empty(area);
      let mut cell = board.get_ref_cell(5, col).clone();
      cell.set_zoom_level(zoom);
      cell.render(area, &mut buffer);
      buffer
    };
    let text =
      |buffer: &Buffer| -> String { buffer.content.iter().map(|cell| cell.symbol()).collect() };
    assert!(!text(&render(&game.board, 2)).contains('%'));
    advance(&mut game, 46);
    let buffer = render(&game.board, 2);
    assert!(text(&buffer).contains("50%"), "{zoom:?}");
    if zoom != ZoomLevel::ZoomedOut {
      let marker = buffer
        .content
        .iter()
        .find(|cell| cell.symbol() == "M")
        .unwrap();
      assert_eq!(marker.fg, Players::Player.get_color());
    }
    advance(&mut game, travel_ticks() - 46);
    assert!(!text(&render(&game.board, 2)).contains('%'));
    assert!(!text(&render(&game.board, 3)).contains('%'));
    game.tick_update();
    assert!(text(&render(&game.board, 3)).contains("1%"));
    advance(&mut game, travel_ticks() - 1);
    assert!(!text(&render(&game.board, 4)).contains('%'));
    assert!(unit(&game.board, 5, 4).get_ref_tasks().is_empty());
  }
}

#[test]
fn confirmed_route_splits_into_moving_and_digging_segments() {
  let mut game = game();
  for col in [3, 4, 7] {
    tunnel(&mut game.board, 5, col);
  }
  game.board.change_focused_cell(MoveFocusedCell::Left);
  game.board.toggle_marking();
  for _ in 0..5 {
    game.board.change_focused_cell(MoveFocusedCell::Right);
  }
  game.board.toggle_marking();
  let tasks = unit(&game.board, 5, 2).get_ref_tasks();
  assert_eq!(tasks.len(), 3);
  assert!(matches!(tasks[0], Tasks::Moving(_)));
  assert_eq!(
    tasks[0].get_path().iter().copied().collect::<Vec<_>>(),
    [(5, 3), (5, 4)]
  );
  assert!(matches!(tasks[1], Tasks::Digging(_)));
  assert_eq!(
    tasks[1].get_path().iter().copied().collect::<Vec<_>>(),
    [(5, 5), (5, 6)]
  );
  assert!(matches!(tasks[2], Tasks::Moving(_)));
  assert_eq!(tasks[0].get_icon(), '→');
  assert_eq!(tasks[1].get_icon(), '⛏');
  advance(&mut game, 2 * travel_ticks());
  assert!(matches!(
    unit(&game.board, 5, 4).get_ref_tasks()[0],
    Tasks::Digging(_)
  ));
  advance(&mut game, 2 * 182);
  assert!(matches!(
    unit(&game.board, 5, 6).get_ref_tasks()[0],
    Tasks::Moving(_)
  ));
  advance(&mut game, travel_ticks());
  assert!(unit(&game.board, 5, 7).get_ref_tasks().is_empty());
}

#[test]
fn moving_task_does_not_damage_walkable_structure() {
  let mut game = game();
  tunnel(&mut game.board, 5, 3);
  let structure = game.board.cells[5][3].get_structure();
  game.board.cells[5][2]
    .get_mut_option_unit()
    .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
  advance(&mut game, travel_ticks());
  assert_eq!(game.board.cells[5][3].get_structure(), structure);
  assert!(unit(&game.board, 5, 3).get_ref_tasks().is_empty());
}

#[test]
fn previously_dug_route_becomes_moving_without_instant_steps() {
  let mut game = game();
  queue(&mut game.board, (5, 2), &[(5, 3), (5, 4), (5, 3)]);
  advance(&mut game, 2 * 182);
  assert_eq!(unit(&game.board, 5, 4).get_coords(), (5, 4));
  game.tick_update();
  assert!(matches!(
    unit(&game.board, 5, 4).get_ref_tasks()[0],
    Tasks::Moving(_)
  ));
  advance(&mut game, travel_ticks() - 2);
  assert_eq!(unit(&game.board, 5, 4).get_coords(), (5, 4));
  game.tick_update();
  assert!(unit(&game.board, 5, 3).get_ref_tasks().is_empty());
}

#[test]
fn route_reclassifies_as_digging_if_target_is_stone() {
  let mut game = game();
  game.board.cells[5][2]
    .get_mut_option_unit()
    .set_task(Tasks::Moving(MovingTask::new([(5, 3)].into())));
  game.tick_update();
  assert!(matches!(
    unit(&game.board, 5, 2).get_ref_tasks()[0],
    Tasks::Digging(_)
  ));
  assert!(game.board.cells[5][3].get_structure().get_durability() < 1000);
  advance(&mut game, 181);
  assert!(unit(&game.board, 5, 3).get_ref_tasks().is_empty());
}

#[test]
fn tab_panel_shows_current_step_and_total_for_active_and_pending_tasks() -> () {
  let args: Cli = Cli::try_parse_from(["game", "--disable-audio", "--view", "skirmish"]).unwrap();
  let channels: (Sender<AudioCmd>, Receiver<AudioCmd>) = channel();
  let mut app: App = App::new(args, channels.0);
  app.window_area = Rect::new(0, 0, 120, 60);
  app.states = Some(GameStates::new(&app.args, &app.window_area));
  let state: &mut SkirmishState = &mut app.states_mut().unwrap().skirmish;
  tunnel(&mut state.board, 5, 3);
  tunnel(&mut state.board, 5, 4);
  state.board.cells[5][2]
    .get_mut_option_unit()
    .set_task(Tasks::Moving(MovingTask::new([(5, 3), (5, 4)].into())));
  state.board.cells[5][2]
    .get_mut_option_unit()
    .set_task(Tasks::Digging(DiggingTask::new([(5, 5)].into(), 0.55)));
  advance(state, travel_ticks() + 46);
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
  let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(120, 60)).unwrap();
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
    .map(|cell: &ratatui::buffer::Cell| -> &str { cell.symbol() })
    .collect();
  assert!(text.contains("Step: 50% | Total: 75%"));
  assert!(text.contains("Step: 0% | Total: 0%"));
}

#[test]
fn space_route_tick_and_tui_show_remaining_durability_at_every_zoom() {
  let args = Cli::try_parse_from(["game", "--disable-audio", "--view", "skirmish"]).unwrap();
  let (tx, _) = channel();
  let mut app = App::new(args, tx);
  app.window_area = Rect::new(0, 0, 120, 40);
  app.states = Some(GameStates::new(&app.args, &app.window_area));
  for code in [
    KeyCode::Left,
    KeyCode::Char(' '),
    KeyCode::Right,
    KeyCode::Char(' '),
  ] {
    handle_keybindings(&mut app, KeyEvent::new(code, KeyModifiers::NONE));
  }
  app.states_mut().unwrap().skirmish.tick_update();
  let durability = app.states_ref().unwrap().skirmish.board.cells[5][3]
    .get_structure()
    .get_durability();
  let mut terminal = Terminal::new(TestBackend::new(120, 40)).unwrap();
  for zoom in [
    ZoomLevel::ZoomedIn,
    ZoomLevel::Default,
    ZoomLevel::ZoomedOut,
  ] {
    let state = &mut app.states_mut().unwrap().skirmish;
    state.board.change_zoom(zoom);
    state.board.vertical_offset.set_value(5);
    terminal
      .draw(|frame| frame.render_widget(&mut app, frame.area()))
      .unwrap();
    let text: String = terminal
      .backend()
      .buffer()
      .content
      .iter()
      .map(|cell| cell.symbol())
      .collect();
    assert!(text.contains("99%"), "{zoom:?}");
  }
  handle_keybindings(&mut app, KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
  terminal
    .draw(|frame| frame.render_widget(&mut app, frame.area()))
    .unwrap();
  let text: String = terminal
    .backend()
    .buffer()
    .content
    .iter()
    .map(|cell| cell.symbol())
    .collect();
  assert!(text.contains(&format!("{durability}/1000")));
  assert_eq!(app.view, View::Skirmish);
}
