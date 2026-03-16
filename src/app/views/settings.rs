use crate::app::{App, keybindings::Action, widgets::KeybindingsWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

pub fn settings_view(app: &App, area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout = Layout::vertical([
        Constraint::Length(4),
        Constraint::Fill(1),
        Constraint::Length(5),
    ]);

    let [title_area, main_area, keybindings_area] = vertical_layout.areas(area);

    {
        Paragraph::new("War in tunnels")
            .alignment(Alignment::Left)
            .yellow()
            .block(
                Block::new()
                    .gray()
                    .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
                    .padding(Padding::new(5, 1, 1, 1)),
            )
            .render(title_area, buf);

        let lines: Vec<Line<'_>> = app
            .states
            .settings
            .options
            .iter()
            .enumerate()
            .map(|(i, o)| {
                if i == app.states.settings.selected_setting {
                    Line::from_iter([
                        "> ".cyan(),
                        o.name.yellow(),
                        ": ".gray(),
                        o.value.to_string().light_green(),
                    ])
                } else {
                    Line::from(format!("  {}: {}", o.name, o.value)).white()
                }
            })
            .collect();

        let settings_block: Block<'_> = Block::new()
            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
            .padding(Padding::new(1, 1, 1, 1))
            .title(Line::from_iter([
                "[ ".gray(),
                "Settings".magenta(),
                " ]".gray(),
            ]))
            .gray();

        let settings_area: Rect = settings_block.inner(main_area);

        settings_block.render(main_area, buf);

        Paragraph::new(lines).alignment(Alignment::Center).render(
            settings_area.centered_vertically(Constraint::Ratio(1, 2)),
            buf,
        );

        let popup_text: Vec<Line<'_>> = Vec::from_iter([
            app.states
                .settings
                .selected_setting_new_value
                .clone()
                .into(),
            app.states.settings.error_message.clone().into(),
        ]);

        if app.states.settings.show_popup {
            Paragraph::new(popup_text)
                .wrap(Wrap { trim: true })
                .style(Style::default())
                .block(
                    Block::new()
                        .title("Insert new value")
                        .title_style(Style::default().yellow())
                        .borders(Borders::ALL)
                        .border_style(Style::default().gray())
                        .padding(Padding {
                            left: 1,
                            right: 1,
                            top: 1,
                            bottom: 1,
                        }),
                )
                .render(
                    Rect {
                        x: 2,
                        y: main_area.height - 1,
                        width: main_area.width - 3,
                        height: if app.states.settings.error_message.is_empty() {
                            5
                        } else {
                            6
                        },
                    },
                    buf,
                );
        }
    }

    {
        let actions: Vec<Action> = vec![
            Action::Up,
            Action::Down,
            Action::Quit,
            Action::Quit2,
            Action::Esc,
            Action::Space,
            Action::Enter,
            Action::Backspace,
            Action::WildCard('_'),
        ];

        KeybindingsWidget::new(actions).render(keybindings_area, buf);
    }
}
