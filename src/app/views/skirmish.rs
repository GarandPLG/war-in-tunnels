use crate::app::{
    App,
    helpers::block_title_helper,
    keybindings::{Action, count_largest_group},
    states::skirmish_states::{structures::Structure, units::Unit},
    widgets::{BoardWidget, KeybindingsWidget, SidePanelWidget},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

const ACTIONS: &[Action] = &[
    Action::Up,
    Action::Down,
    Action::Left,
    Action::Right,
    Action::ScrollUp,
    Action::ScrollDown,
    Action::ScrollLeft,
    Action::ScrollRight,
    Action::ZoomIn,
    Action::ZoomOut,
    Action::VolumeUp,
    Action::VolumeDown,
    Action::Space,
    Action::Backspace,
    Action::Delete,
    Action::Mute,
    Action::Tab,
    Action::Quit,
    Action::Quit2,
    Action::Esc,
];

pub fn skirmish_layout(area: Rect) -> [Rect; 3] {
    Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(count_largest_group(&ACTIONS) + 2),
    ])
    .areas(area)
}

pub fn skirmish_main_area_layout(area: Rect, side_panel: bool) -> [Rect; 2] {
    let constraint: [Constraint; 2] = if side_panel {
        [Constraint::Percentage(80), Constraint::Percentage(20)]
    } else {
        [Constraint::Percentage(100), Constraint::Percentage(0)]
    };

    Layout::horizontal(constraint).areas(area)
}

pub fn skirmish_view(app: &App, area: Rect, buf: &mut Buffer) {
    let Some(states) = app.states() else { return };

    let [title_area, main_area, keybindings_area] = skirmish_layout(area);

    {
        let lines: Vec<Line<'_>> = Vec::from_iter([
            Line::raw("War in Tunnels").yellow(),
            Line::from_iter([
                format!("Wood: {} (+{}) | ", states.settings.starting_wood, 5),
                format!("Iron: {} (+{}) | ", states.settings.starting_iron, 1),
                format!("Supply Limit: {}/{} | ", 10, states.settings.supply_limit),
                format!(
                    "Skills points: {} ({}/{}) | ",
                    1, 20, states.settings.skill_points_limit
                ),
                format!("Perk Deck: {}/9", 5),
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
        //         (states.skirmish.board.cell_width * states.skirmish.board.cols) as u16,
        //     ),
        //     Constraint::Length(
        //         (states.skirmish.board.cell_height * states.skirmish.board.rows) as u16,
        //     ),
        // );

        BoardWidget::new(&states.skirmish.board).render(cells_area, buf);

        if states.skirmish.side_panel {
            let row: usize = states.skirmish.board.get_focused_cell().get_row();
            let col: usize = states.skirmish.board.get_focused_cell().get_col();
            let coords: (usize, usize) = (row, col);
            let structure_name: &str = states
                .skirmish
                .board
                .get_ref_cell(row, col)
                .get_structure()
                .get_name();
            let unit_name: &str = states
                .skirmish
                .board
                .get_ref_cell(row, col)
                .get_unit_option()
                .get_name();

            SidePanelWidget::new(coords, structure_name, unit_name).render(side_panel_area, buf);
        }
    }

    {
        KeybindingsWidget::new(ACTIONS, keybindings_area.height).render(keybindings_area, buf);
    }
}
