use crate::app::{
    App,
    keybindings::Action,
    widgets::{BoardWidget, KeybindingsWidget},
};
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
        Constraint::Length(6),
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

        let cells_area: Rect = board_area.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        BoardWidget::new(app, cells_area.width, cells_area.height).render(cells_area, buf);
    }

    {
        let actions: Vec<Action> = vec![
            Action::ScrollUp,
            Action::ScrollDown,
            Action::ScrollLeft,
            Action::ScrollRight,
            Action::Quit,
            Action::Quit2,
            Action::Esc,
        ];

        KeybindingsWidget::new(actions).render(keybindings_area, buf);
    }
}
