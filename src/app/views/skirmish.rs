use crate::app::{App, keybindings::Action, widgets::KeybindingsWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Margin, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub fn skirmish_view(app: &App, area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(4),
    ]);

    let [title_area, main_area, keybindings_area] = vertical_layout.areas(area);

    {
        let lines: Vec<Line<'_>> = Vec::from_iter([
            Line::raw("War in Tunnels").yellow(),
            Line::from_iter([
                format!("Wood: {} (+{}) | ", app.states.settings.starting_wood, 5),
                format!("Iron: {} (+{}) | ", app.states.settings.starting_iron, 1),
                format!(
                    "Supply Limit: {}/{} | ",
                    10, app.states.settings.supply_limit
                ),
                format!(
                    "Skills points: {} ({}/{}) | ",
                    1, 20, app.states.settings.skill_points_limit
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
        let board_block: Block = Block::new()
            .gray()
            .title(Line::from_iter([
                "[ ".gray(),
                "Skirmish".magenta(),
                " - ".gray(),
                "Map".green(),
                " ]".gray(),
            ]))
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT);

        let board_area: Rect = board_block.inner(main_area);
        board_block.render(main_area, buf);

        let inner_board_area: Rect = board_area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        let map_width: u16 = inner_board_area.width / (app.states.settings.map_width as u16);
        let map_height: u16 = inner_board_area.height / (app.states.settings.map_height as u16);

        Paragraph::new(format!(
            "x = {}, y = {}, mw = {}, mh = {}",
            inner_board_area.width, inner_board_area.height, map_width, map_height
        ))
        .block(Block::default())
        .render(inner_board_area, buf);
    }

    {
        let actions: Vec<Action> = vec![Action::Quit, Action::Quit2, Action::Esc];

        KeybindingsWidget::new(actions).render(keybindings_area, buf);
    }
}
