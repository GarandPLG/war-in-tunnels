use crate::app::{
    keybindings::{Action, count_largest_group},
    widgets::KeybindingsWidget,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

const ACTIONS: &[Action] = &[Action::Quit, Action::Quit2, Action::Esc];

fn default_view_layout(area: Rect) -> [Rect; 2] {
    Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(count_largest_group(&ACTIONS) + 2),
    ])
    .areas(area)
}

pub fn default_view(area: Rect, buf: &mut Buffer) {
    let [main_area, keybindings_area] = default_view_layout(area);

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
        KeybindingsWidget::new(ACTIONS, keybindings_area.height).render(keybindings_area, buf);
    }
}
