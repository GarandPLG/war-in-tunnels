use crate::app::{App, keybindings::Action, widgets::KeybindingsWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap},
};

pub fn settings_view(app: &App, area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout = Layout::vertical([
        Constraint::Length(10),
        Constraint::Fill(1),
        Constraint::Length(4),
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
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .render(title_area, buf);

        Paragraph::new("Settings")
            .alignment(Alignment::Center)
            .yellow()
            .block(
                Block::new()
                    .gray()
                    .borders(Borders::LEFT | Borders::RIGHT)
                    .padding(Padding::new(1, 1, 1, 1)),
            )
            .render(main_area, buf);

        if app.states.settings.show_popup {
            Paragraph::new(app.states.settings.selected_setting_new_value.clone())
                .wrap(Wrap { trim: true })
                .style(Style::default().yellow())
                .block(
                    Block::new()
                        .title("Insert value")
                        .title_style(Style::default().green())
                        .borders(Borders::ALL)
                        .border_style(Style::default().blue())
                        .padding(Padding {
                            left: 1,
                            right: 1,
                            top: 1,
                            bottom: 1,
                        }),
                )
                .render(
                    Rect {
                        x: main_area.width / 3,
                        y: main_area.height / 2,
                        width: main_area.width / 3,
                        height: main_area.height / 7,
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
