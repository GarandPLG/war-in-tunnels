use crate::app::{
    App,
    helpers::block_title_helper,
    keybindings::{Action, available_skirmish_actions},
    states::SkirmishState,
    states::skirmish_states::{
        BoardState, Players,
        resources::{WOOD_DELIVERY_AMOUNT, WOOD_DELIVERY_INTERVAL_MS},
        structures::{Mine, Structure, Structures, Tunnel},
        units::MinerUnit,
    },
    widgets::{BoardWidget, CellWidget, KeybindingsWidget, SidePanelWidget},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Padding, Paragraph, Tabs, Widget},
};

pub fn skirmish_layout(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
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
            Line::raw("War in Tunnels").fg(Color::Rgb(128, 128, 0)),
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
                    .fg(Color::Rgb(192, 192, 192))
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
            .fg(Color::Rgb(192, 192, 192))
            .title(block_title_helper(
                &[
                    ("Skirmish".to_string(), Color::Rgb(128, 0, 128)),
                    ("Map".to_string(), Color::Rgb(0, 128, 0)),
                ],
                Some(" - "),
            ))
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT);

        let main_inner_area: Rect = main_block.inner(main_area);
        main_block.render(main_area, buf);

        let [board_area, side_panel_area] = skirmish_main_area_layout(
            main_inner_area,
            states.skirmish.side_panel || states.skirmish.task_view,
        );

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

        BoardWidget::new(&board, states.skirmish.stress_view).render(cells_area, buf);

        if states.skirmish.task_view {
            let mut lines: Vec<Line<'_>> = vec![
                Line::from("Enter: draw route").fg(Color::Rgb(0, 128, 0)),
                Line::from(format!(
                    "Auto supports: {}",
                    if board.auto_supports { "ON" } else { "OFF" }
                ))
                .fg(Color::Rgb(0, 128, 128)),
                Line::from("a: toggle auto supports").fg(Color::Rgb(128, 128, 0)),
                Line::from("Support: 10 Wood + 2 Metal").fg(Color::Rgb(128, 128, 0)),
                Line::from("Placed at 80% stress").fg(Color::Rgb(192, 192, 192)),
                Line::from("No funds: collapse risk!").fg(Color::Rgb(128, 0, 0)),
                Line::from("t: close task selection").fg(Color::Rgb(192, 192, 192)),
            ];
            if states.skirmish.selected_task == 1 {
                lines = vec![
                    Line::from("Enter: draw movement route").fg(Color::Rgb(0, 128, 0)),
                    Line::from("Walkable cells only; no digging").fg(Color::Rgb(0, 128, 128)),
                    Line::from("Collapsed route: wait, do not dig").fg(Color::Rgb(192, 192, 192)),
                    Line::from("t: close task selection").fg(Color::Rgb(192, 192, 192)),
                ];
            }
            let block: Block<'_> = Block::bordered()
                .title("Unit tasks")
                .padding(Padding::uniform(1));
            let [tabs_area, details_area]: [Rect; 2] =
                Layout::vertical([Constraint::Length(2), Constraint::Min(0)])
                    .areas(block.inner(side_panel_area));
            block.render(side_panel_area, buf);
            Tabs::new(SkirmishState::TASK_OPTIONS.iter().copied())
                .select(states.skirmish.selected_task)
                .highlight_style(Style::default().fg(Color::Rgb(255, 0, 255)).bold())
                .render(tabs_area, buf);
            Paragraph::new(lines).render(details_area, buf);
        } else if states.skirmish.side_panel {
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
        let descriptions: [(Action, &'static str); 17] = [
            (
                Action::Tasks,
                if states.skirmish.task_view {
                    "Close tasks"
                } else {
                    "Unit tasks"
                },
            ),
            (
                Action::AutoSupports,
                if board.auto_supports {
                    "Auto supports: ON"
                } else {
                    "Auto supports: OFF"
                },
            ),
            (
                Action::Stress,
                if states.skirmish.stress_view {
                    "Normal view"
                } else {
                    "Stress view"
                },
            ),
            (
                Action::Left,
                if states.skirmish.building_view
                    || states.skirmish.recruitment_view
                    || states.skirmish.task_view
                {
                    "Previous option"
                } else {
                    "Left"
                },
            ),
            (
                Action::Right,
                if states.skirmish.building_view
                    || states.skirmish.recruitment_view
                    || states.skirmish.task_view
                {
                    "Next option"
                } else {
                    "Right"
                },
            ),
            (
                Action::Backspace,
                if board.marked_cells.marking_cells {
                    "Undo last cell"
                } else {
                    "Cancel last task"
                },
            ),
            (
                Action::Enter,
                if states.skirmish.task_view {
                    "Draw route"
                } else if states.skirmish.recruitment_view {
                    "Queue Miner"
                } else {
                    if states.skirmish.selected_building == 1 {
                        "Build support"
                    } else {
                        "Build Mine"
                    }
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
                    if states.skirmish.selected_building == 1 {
                        "Remove support"
                    } else {
                        "Cancel construction"
                    }
                } else if !board.marked_cells.marking_cells {
                    "Cancel all tasks"
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
                if board.marked_cells.marking_cells {
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
        let status: Vec<Line<'_>> = if let Structures::MineConstruction(site) = structure
            && site.get_owner() == Players::Player
        {
            vec![
                Line::from(format!("Construction: {}%", site.progress_percent()))
                    .fg(Color::Rgb(0, 128, 128)),
                if board.has_adjacent_builder(coords, Players::Player) {
                    Line::from("Builder working").fg(Color::Rgb(0, 128, 0))
                } else {
                    Line::from("Paused: adjacent friendly builder required")
                        .fg(Color::Rgb(128, 128, 0))
                },
                Line::from("Delete: cancel and refund full cost").fg(Color::Rgb(128, 128, 0)),
            ]
        } else if matches!(structure, Structures::Mine(_))
            && structure.get_owner() == Players::Player
        {
            vec![Line::from("Mine completed").fg(Color::Rgb(0, 128, 0))]
        } else if states
            .skirmish
            .can_start_mine_construction(coords, Players::Player)
        {
            vec![Line::from("Ready to build: Enter").fg(Color::Rgb(0, 128, 0))]
        } else {
            vec![
                Line::from("Construction unavailable: check site, builder and resources")
                    .fg(Color::Rgb(128, 0, 0)),
            ]
        };
        let mut lines: Vec<Line<'_>> = vec![
            Line::from(format!(
                "Cost: {} Wood + {} Metal | Build time: 10s",
                Mine::COST.wood,
                Mine::COST.metal
            ))
            .fg(Color::Rgb(128, 128, 0)),
            Line::from("Requires: neutral Ore + adjacent friendly builder")
                .fg(Color::Rgb(192, 192, 192)),
            Line::from("Produces: 1 Metal/s (2 with a Miner inside)").fg(Color::Rgb(0, 128, 128)),
        ];
        lines.extend(status);
        if states.skirmish.selected_building == 1 {
            lines = vec![
                Line::from(format!(
                    "Cost: {} Wood + {} Metal | Work time: 3s",
                    Tunnel::SUPPORT_COST.wood,
                    Tunnel::SUPPORT_COST.metal
                ))
                .fg(Color::Rgb(128, 128, 0)),
                Line::from("Tunnel + nearby friendly builder required")
                    .fg(Color::Rgb(192, 192, 192)),
                Line::from("Stress: 20% at support, +20% per tile").fg(Color::Rgb(0, 128, 128)),
                if matches!(structure, Structures::Tunnel(tunnel) if tunnel.support_work().is_some())
                {
                    Line::from("Builder occupied until completion").fg(Color::Rgb(128, 128, 0))
                } else if matches!(structure, Structures::Tunnel(tunnel) if tunnel.has_support())
                    && states.skirmish.can_manage_support(coords, Players::Player)
                {
                    Line::from("Delete: remove support (no refund)").fg(Color::Rgb(128, 128, 0))
                } else if states.skirmish.can_build_support(coords, Players::Player) {
                    Line::from("Enter: build support").fg(Color::Rgb(0, 128, 0))
                } else {
                    Line::from("Support unavailable: site, builder or resources")
                        .fg(Color::Rgb(128, 0, 0))
                },
                Line::from("WARNING: removal can cause fatal collapses").fg(Color::Rgb(128, 0, 0)),
            ];
            if let Structures::Tunnel(tunnel) = structure
                && let Some(work) = tunnel.support_work()
            {
                lines.push(
                    Line::from(format!(
                        "{}: {}%",
                        if work.removing {
                            "Removing support"
                        } else {
                            "Building support"
                        },
                        tunnel.support_progress().unwrap_or(0)
                    ))
                    .fg(Color::Rgb(0, 128, 128)),
                );
                if board.support_worker(coords, work.owner).is_none() {
                    lines.push(
                        Line::from("Paused: friendly builder required").fg(Color::Rgb(128, 128, 0)),
                    );
                }
            }
        }
        Clear.render(popup, buf);
        let block: Block<'_> = Block::bordered()
            .title(Line::from("Buildings").fg(Color::Rgb(128, 0, 128)).bold())
            .padding(Padding::uniform(1));
        let [tabs_area, details_area]: [Rect; 2] =
            Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(block.inner(popup));
        block.render(popup, buf);
        Tabs::new(SkirmishState::BUILDING_OPTIONS.iter().copied())
            .select(states.skirmish.selected_building)
            .highlight_style(Style::default().fg(Color::Rgb(255, 0, 255)).bold())
            .render(tabs_area, buf);
        Paragraph::new(lines).render(details_area, buf);
    }
    if states.skirmish.recruitment_view {
        let popup: Rect = main_area.centered(Constraint::Length(64), Constraint::Length(14));
        let coords: (usize, usize) = board.get_focused_cell().get_coords();
        let cell: &CellWidget = board.get_ref_cell(coords.0, coords.1);
        let lines: Vec<Line<'_>> = if let Structures::Base(base) = cell.get_structure()
            && base.get_owner() == Players::Player
        {
            let status: &'static str =
                base.recruitment_status(cell.get_ref_option_unit().is_some());
            let action: Line<'_> = if states.skirmish.can_recruit_miner(coords, Players::Player) {
                Line::from("Enter: queue Miner").fg(Color::Rgb(0, 128, 0))
            } else {
                Line::from("Insufficient resources or queue full").fg(Color::Rgb(128, 0, 0))
            };
            vec![
                Line::from(format!(
                    "Cost: {} Wood + {} Metal | Recruit time: 10s",
                    MinerUnit::RECRUIT_COST.wood,
                    MinerUnit::RECRUIT_COST.metal
                ))
                .fg(Color::Rgb(128, 128, 0)),
                Line::from(format!(
                    "Queued Miners: {} (includes active)",
                    base.queued_miners()
                ))
                .fg(Color::Rgb(0, 128, 128)),
                Line::from(format!(
                    "Current progress: {}%",
                    base.recruitment_progress()
                ))
                .fg(Color::Rgb(0, 128, 128)),
                Line::from(status).fg(Color::Rgb(128, 128, 0)),
                action,
            ]
        } else {
            vec![Line::from("Select your Base to recruit Miners").fg(Color::Rgb(128, 0, 0))]
        };
        Clear.render(popup, buf);
        let block: Block<'_> = Block::bordered()
            .title(Line::from("Recruitment").fg(Color::Rgb(128, 0, 128)).bold())
            .padding(Padding::uniform(1));
        let [tabs_area, details_area]: [Rect; 2] =
            Layout::vertical([Constraint::Length(2), Constraint::Min(0)]).areas(block.inner(popup));
        block.render(popup, buf);
        Tabs::new(SkirmishState::RECRUITMENT_OPTIONS.iter().copied())
            .select(states.skirmish.selected_recruit)
            .highlight_style(Style::default().fg(Color::Rgb(255, 0, 255)).bold())
            .render(tabs_area, buf);
        Paragraph::new(lines).render(details_area, buf);
    }
}
