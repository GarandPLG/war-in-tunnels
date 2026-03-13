use crate::app::{
    App,
    keybindings::{Action, KeyBinding, binding_for},
    widgets::KeybindingsWidget,
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
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

    {
        let keybindings: Vec<Option<&'static KeyBinding>> = vec![
            binding_for(Action::Up),
            binding_for(Action::Down),
            binding_for(Action::Space),
            binding_for(Action::Enter),
            binding_for(Action::Quit),
            binding_for(Action::Quit2),
            binding_for(Action::Esc),
        ];

        KeybindingsWidget::new(keybindings).render(keybindings_area, buf);
    }
}
