use crate::app::{keybindings::Action, widgets::KeybindingsWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn default_view(area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]);

    let [main_area, keybindings_area] = vertical_layout.areas(area);

    {
        Paragraph::new("Work in progress")
            .alignment(Alignment::Center)
            .yellow()
            .block(
                Block::new()
                    .gray()
                    .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT),
            )
            .render(main_area, buf);
    }

    {
        let actions: Vec<Action> = vec![Action::Quit, Action::Quit2, Action::Esc];

        KeybindingsWidget::new(actions).render(keybindings_area, buf);
    }
}
