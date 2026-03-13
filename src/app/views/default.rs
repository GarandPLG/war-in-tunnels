use crate::app::{
    keybindings::{Action, KeyBinding, binding_for},
    widgets::KeybindingsWidget,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn default_view(area: Rect, buf: &mut Buffer) {
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

    {
        let keybindings: Vec<Option<&'static KeyBinding>> = vec![
            binding_for(Action::Quit),
            binding_for(Action::Quit2),
            binding_for(Action::Esc),
        ];

        KeybindingsWidget::new(keybindings).render(keybindings_area, buf);
    }
}
