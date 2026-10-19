use clap::Parser;
use ratatui::{Terminal, backend::TestBackend, buffer::Cell, layout::Rect};
use std::sync::mpsc::{Receiver, Sender, channel};
use war_in_tunnels::{
    app::{
        App, GameStates, View,
        states::{
            SkirmishState,
            skirmish_states::{
                Players, Resources,
                resources::{MINE_PRODUCTION_TICKS, WOOD_DELIVERY_AMOUNT, WOOD_DELIVERY_TICKS},
                structures::{Mine, Ore, Stone, Structure, Structures},
            },
        },
        threads::AudioCmd,
    },
    cli::Cli,
};

fn game() -> SkirmishState {
    let args: Cli =
        Cli::try_parse_from(["game", "--starting-wood", "70", "--starting-metal", "40"]).unwrap();
    GameStates::new(&args, &Rect::new(0, 0, 160, 40)).skirmish
}

#[test]
fn resources_start_from_cli_and_legacy_iron_flag_remains_an_alias() -> () {
    let game: SkirmishState = game();
    assert_eq!(game.player_resources, Resources::new(70, 40));
    assert_eq!(game.enemy_resources, Resources::new(70, 40));
    let args: Cli = Cli::try_parse_from(["game", "--starting-iron", "33"]).unwrap();
    assert_eq!(args.starting_metal, 33);
}

#[test]
fn passive_wood_arrives_once_per_interval_without_free_metal() -> () {
    let mut game: SkirmishState = game();
    for _ in 1..WOOD_DELIVERY_TICKS {
        game.tick_update();
    }
    assert_eq!(game.player_resources.wood, 70);
    game.tick_update();
    assert_eq!(game.player_resources.wood, 70 + WOOD_DELIVERY_AMOUNT);
    for _ in 0..WOOD_DELIVERY_TICKS {
        game.tick_update();
    }
    assert_eq!(game.player_resources.wood, 70 + 2 * WOOD_DELIVERY_AMOUNT);
    assert_eq!(game.player_resources.metal, 40);
    assert_eq!(game.enemy_resources, Resources::new(70, 40));
}

#[test]
fn ore_is_neutral_and_no_mines_spawn_on_the_board() -> () {
    let game: SkirmishState = game();
    let mut deposits: usize = 0;
    for row in &game.board.cells {
        for cell in row {
            let structure: Structures = cell.get_structure();
            assert!(!matches!(structure, Structures::Mine(_)));
            if matches!(structure, Structures::Ore(_)) {
                deposits += 1;
                assert_eq!(structure.get_owner(), Players::Unclaimed);
            }
        }
    }
    assert_eq!(deposits, 2);
}

#[test]
fn mine_can_only_replace_neutral_ore_and_preserves_its_reserve() -> () {
    let mut ore: Ore = Ore::new(Players::Unclaimed, 1);
    assert_eq!(ore.extract(4999), 4999);
    let mut structure: Structures = Structures::Ore(ore);
    assert!(structure.build_mine(Players::Player));
    let Structures::Mine(mut mine) = structure else {
        panic!("expected mine");
    };
    assert_eq!(mine.get_owner(), Players::Player);
    assert_eq!(mine.get_ore_amount(), 1);
    assert_eq!(mine.produce(), 1);
    assert_eq!(mine.produce(), 0);
    assert_eq!(mine.get_ore_amount(), 0);
    assert!(!structure.build_mine(Players::Enemy));
    let mut stone: Structures = Structures::Stone(Stone::new());
    assert!(!stone.build_mine(Players::Player));
    assert_eq!(stone, Structures::Stone(Stone::new()));
    assert!(Mine::new(Players::Unclaimed, ore).is_none());
    assert!(Mine::new(Players::Player, Ore::new(Players::Enemy, 1)).is_none());
}

#[test]
fn mining_credits_each_owner_and_stops_when_deposit_is_empty() -> () {
    let mut game: SkirmishState = game();
    for (col, owner) in [(1, Players::Player), (40, Players::Enemy)] {
        let mut ore: Ore = Ore::new(Players::Unclaimed, 1);
        ore.extract(4999);
        let mut structure: Structures = Structures::Ore(ore);
        assert!(structure.build_mine(owner));
        game.board.cells[5][col].set_structure(structure);
    }
    for _ in 1..MINE_PRODUCTION_TICKS {
        game.tick_update();
    }
    assert_eq!(game.player_resources.metal, 40);
    game.tick_update();
    assert_eq!(game.player_resources.metal, 41);
    assert_eq!(game.enemy_resources.metal, 41);
    for _ in 0..MINE_PRODUCTION_TICKS {
        game.tick_update();
    }
    assert_eq!(game.player_resources.metal, 41);
    assert_eq!(game.enemy_resources.metal, 41);
    let structure: Structures = game.board.cells[5][1].get_structure();
    assert!(
        structure
            .text()
            .iter()
            .any(|line: &ratatui::text::Line<'_>| -> bool {
                line.to_string().contains("Ore: 0/5000")
            })
    );
}

#[test]
fn resource_additions_do_not_overflow() -> () {
    let mut resources: Resources = Resources {
        wood: u32::MAX - 1,
        metal: u32::MAX - 1,
    };
    resources.add_wood(5);
    resources.add_metal(5);
    assert_eq!(resources.wood, u32::MAX);
    assert_eq!(resources.metal, u32::MAX);
}

#[test]
fn tui_reads_current_resources_instead_of_starting_settings() -> () {
    let args: Cli = Cli::try_parse_from(["game", "--disable-audio"]).unwrap();
    let (tx, _rx): (Sender<AudioCmd>, Receiver<AudioCmd>) = channel();
    let mut app: App = App::new(args, tx);
    app.view = View::Skirmish;
    app.states = Some(GameStates::new(&app.args, &Rect::new(0, 0, 200, 40)));
    app.states_mut().unwrap().skirmish.player_resources = Resources {
        wood: 777,
        metal: 888,
    };
    let mut terminal: Terminal<TestBackend> = Terminal::new(TestBackend::new(200, 40)).unwrap();
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
    assert!(text.contains("Wood: 777 (+5/5s)"));
    assert!(text.contains("Metal: 888"));
    assert!(!text.contains("Iron:"));
}
