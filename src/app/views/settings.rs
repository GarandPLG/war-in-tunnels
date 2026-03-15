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

    let popup_area = Rect {
        x: main_area.width / 4,
        y: main_area.height / 3,
        width: main_area.width / 2,
        height: main_area.height / 3,
    };

    let popup = Paragraph::new("Test popup")
        .wrap(Wrap { trim: true })
        .style(Style::default().yellow())
        .block(
            Block::new()
                .title("Insert value")
                .title_style(Style::default().green())
                .borders(Borders::ALL)
                .border_style(Style::default().blue()),
        );

    if app.game_states.settings_state.show_popup {
        popup.render(popup_area, buf);
    }

    {
        let actions: Vec<Action> = vec![
            Action::Up,
            Action::Down,
            Action::Space,
            Action::Enter,
            Action::Quit,
            Action::Quit2,
            Action::Esc,
        ];

        KeybindingsWidget::new(actions).render(keybindings_area, buf);
    }
}
