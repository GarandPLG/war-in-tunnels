use crate::app::{App, widgets::KeybindingsWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn default_view_widget(app: &App, area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]);

    let [main_area, keybindings_area] = vertical_layout.areas(area);

    Paragraph::new("Work in progress")
        .alignment(Alignment::Center)
        .yellow()
        .block(
            Block::new()
                .gray()
                .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT),
        )
        .render(main_area, buf);

    KeybindingsWidget::new(app.view).render(keybindings_area, buf);
}
