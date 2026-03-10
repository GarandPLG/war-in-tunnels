use crate::app::{View, keybindings::binding_for_view};
use ratatui::{
    crossterm::event::KeyCode,
    layout::Alignment,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

pub fn keybindings_widget(view: View) -> Paragraph<'static> {
    let lines: Vec<Line> = binding_for_view(view)
        .iter()
        .map(|b| {
            let key_span = match b.code {
                KeyCode::Up => Span::raw("↑".to_string()),
                KeyCode::Down => Span::raw("↓".to_string()),
                KeyCode::Char(' ') => Span::raw("Space".to_string()),
                KeyCode::Char(c) => Span::raw(c.to_string()),
                other => Span::raw(format!("{:?}", other)),
            }
            .bold()
            .red();

            Line::from_iter([key_span, "\t - ".into(), b.description.into()])
        })
        .collect();

    Paragraph::new(lines).alignment(Alignment::Left).block(
        Block::default()
            .borders(Borders::ALL)
            .title("[ Keybindings ]"),
    )
}
