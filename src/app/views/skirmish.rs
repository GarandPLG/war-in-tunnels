use crate::app::{
    App,
    helpers::block_title_helper,
    keybindings::{Action, available_skirmish_actions},
    states::skirmish_states::{
        BoardState, Players,
        resources::{WOOD_DELIVERY_AMOUNT, WOOD_DELIVERY_INTERVAL_MS},
        structures::{Mine, Structure, Structures},
        units::MinerUnit,
    },
    widgets::{BoardWidget, CellWidget, KeybindingsWidget, SidePanelWidget},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Padding, Paragraph, Widget},
};

pub fn skirmish_layout(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        // Four movement rows plus borders keep the board geometry stable across modes.
        Constraint::Length(6),
    ])
    .areas(area)
}

pub fn skirmish_main_area_layout(area: Rect, side_panel: bool) -> [Rect; 2] {
    let constraint: [Constraint; 2] = if side_panel {
        [Constraint::Fill(1), Constraint::Length(39)]
    } else {
        [Constraint::Percentage(100), Constraint::Percentage(0)]
    };

    Layout::horizontal(constraint).areas(area)
}

pub fn skirmish_view(app: &App, area: Rect, buf: &mut Buffer) {
    let Some(states) = app.states_ref() else {
        return;
    };

    let board: &BoardState = &states.skirmish.board;

    let [title_area, main_area, keybindings_area] = skirmish_layout(area);

    {
        let lines: Vec<Line<'_>> = Vec::from_iter([
            Line::raw("War in Tunnels").yellow(),
            Line::from_iter([
                format!(
                    "Wood: {} (+{}/{}s) | ",
                    states.skirmish.player_resources.wood,
                    WOOD_DELIVERY_AMOUNT,
                    WOOD_DELIVERY_INTERVAL_MS / 1000
                ),
                format!("Metal: {} | ", states.skirmish.player_resources.metal),
                format!("Supply Limit: {}/{} | ", 10, states.settings.supply_limit),
                format!(
                    "Skills points: {} ({}/{}) | ",
                    1, 20, states.settings.skill_points_limit
                ),
                format!("Perk Deck: {}/9 | ", 5),
                format!("Tick: {}", states.skirmish.turn_counter),
            ]),
        ]);

        Paragraph::new(lines)
            .alignment(Alignment::Left)
            .block(
                Block::new()
                    .gray()
                    .padding(Padding {
                        left: 1,
                        right: 1,
                        top: 0,
                        bottom: 1,
                    })
                    .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT),
            )
            .render(title_area, buf);
    }

    {
        let main_block: Block = Block::new()
            .gray()
            .title(block_title_helper(
                &[
                    ("Skirmish".to_string(), Color::Magenta),
                    ("Map".to_string(), Color::Green),
                ],
                Some(" - "),
            ))
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT);

        let main_inner_area: Rect = main_block.inner(main_area);
        main_block.render(main_area, buf);

        let [board_area, side_panel_area] =
            skirmish_main_area_layout(main_inner_area, states.skirmish.side_panel);

        let cells_area: Rect = board_area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });
        // .centered(
        //     Constraint::Length(
        //         (board.cell_width * board.cols) as u16,
        //     ),
        //     Constraint::Length(
        //         (board.cell_height * board.rows) as u16,
        //     ),
        // );

        BoardWidget::new(&board).render(cells_area, buf);

        if states.skirmish.side_panel {
            let (row, col) = board.get_focused_cell().get_coords();
            let mut is_unit_marked: bool = false;

            if let Some((first_row, first_col)) = board.marked_cells.marked_cells.get(0) {
                is_unit_marked = first_row == &row && first_col == &col;
            }

            SidePanelWidget::new(
                (row, col),
                &board.get_ref_cell(row, col).get_structure(),
                &board.get_ref_cell(row, col).get_ref_option_unit(),
                is_unit_marked,
            )
            .render(side_panel_area, buf);
        }
    }

    {
        let actions: Vec<Action> = available_skirmish_actions(&states.skirmish);
        let descriptions: [(Action, &'static str); 11] = [
            (
                Action::Enter,
                if states.skirmish.recruitment_view {
                    "Queue Miner"
                } else {
                    "Build Mine"
                },
            ),
            (
                Action::Recruitment,
                if states.skirmish.recruitment_view {
                    "Close recruitment"
                } else {
                    "Recruit"
                },
            ),
            (
                Action::Delete,
                if states.skirmish.building_view {
                    "Cancel construction"
                } else {
                    "Cancel marking"
                },
            ),
            (
                Action::Space,
                if board.marked_cells.marking_cells {
                    "Confirm route"
                } else {
                    "Select miner"
                },
            ),
            (
                Action::Esc,
                if states.skirmish.recruitment_view {
                    "Close recruitment"
                } else if states.skirmish.building_view {
                    "Close buildings"
                } else if board.marked_cells.marking_cells {
                    "Cancel route"
                } else {
                    "Menu"
                },
            ),
            (
                Action::Building,
                if states.skirmish.building_view {
                    "Close buildings"
                } else {
                    "Build"
                },
            ),
            (Action::Tab, "Details"),
            (Action::ScrollUp, "Scroll"),
            (Action::ScrollDown, "Scroll"),
            (Action::ScrollLeft, "Scroll"),
            (Action::ScrollRight, "Scroll"),
        ];
        KeybindingsWidget::with_descriptions(&actions, keybindings_area.height, &descriptions)
            .render(keybindings_area, buf);
    }
    if states.skirmish.building_view {
        let popup: Rect = main_area.centered(Constraint::Length(64), Constraint::Length(14));
        let coords: (usize, usize) = board.get_focused_cell().get_coords();
        let structure: Structures = board.get_ref_cell(coords.0, coords.1).get_structure();
        let status: String = if let Structures::MineConstruction(site) = structure
            && site.get_owner() == Players::Player
        {
            format!(
                "Construction: {}%\n{}\nDelete: cancel and refund full cost",
                site.progress_percent(),
                if board.has_adjacent_builder(coords, Players::Player) {
                    "Builder working"
                } else {
                    "Paused: adjacent friendly builder required"
                }
            )
        } else if matches!(structure, Structures::Mine(_))
            && structure.get_owner() == Players::Player
        {
            "Mine completed".to_string()
        } else if states
            .skirmish
            .can_start_mine_construction(coords, Players::Player)
        {
            "Ready to build: Enter".to_string()
        } else {
            "Construction unavailable: check site, builder and resources".to_string()
        };
        Clear.render(popup, buf);
        Paragraph::new(format!("Mine\nCost: {} Wood + {} Metal | Build time: 10s\nRequires: neutral Ore + adjacent friendly builder\nProduces: 1 Metal/s (2 with a Miner inside)\n{status}", Mine::COST.wood, Mine::COST.metal))
      .block(Block::bordered().title("Buildings").padding(Padding::uniform(1)))
      .render(popup, buf);
    }
    if states.skirmish.recruitment_view {
        let popup: Rect = main_area.centered(Constraint::Length(64), Constraint::Length(14));
        let coords: (usize, usize) = board.get_focused_cell().get_coords();
        let cell: &CellWidget = board.get_ref_cell(coords.0, coords.1);
        let text: String = if let Structures::Base(base) = cell.get_structure()
            && base.get_owner() == Players::Player
        {
            let status: &'static str =
                base.recruitment_status(cell.get_ref_option_unit().is_some());
            let action: &'static str = if states.skirmish.can_recruit_miner(coords, Players::Player)
            {
                "Enter: queue Miner"
            } else {
                "Insufficient resources or queue full"
            };
            format!(
                "Miner\nCost: {} Wood + {} Metal | Recruit time: 10s\nQueued Miners: {} (includes active)\nCurrent progress: {}%\n{status}\n{action}",
                MinerUnit::RECRUIT_COST.wood,
                MinerUnit::RECRUIT_COST.metal,
                base.queued_miners(),
                base.recruitment_progress()
            )
        } else {
            "Select your Base to recruit Miners".to_string()
        };
        Clear.render(popup, buf);
        Paragraph::new(text)
            .block(
                Block::bordered()
                    .title("Recruitment")
                    .padding(Padding::uniform(1)),
            )
            .render(popup, buf);
    }
}
