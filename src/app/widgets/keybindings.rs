use crate::app::{
    View,
    keybindings::{Group, binding_for_view},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

pub struct KeybindingsWidget {
    grouped: Vec<Paragraph<'static>>,
}

impl Widget for KeybindingsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let count: u16 = self.grouped.len() as u16;

        if count == 0 {
            return;
        }

        let block: Block<'_> = Block::default()
            .borders(Borders::ALL)
            .title("[ Keybindings ]");

        let inner: Rect = block.inner(area);

        block.render(area, buf);

        let base: u16 = if count == 0 { 0 } else { 100u16 / count };

        let constraints: Vec<Constraint> = vec![Constraint::Percentage(base); count as usize];

        let chunks: Vec<Rect> = Layout::horizontal(constraints).split(inner).to_vec();

        for (paragraph, chunk) in self.grouped.into_iter().zip(chunks.into_iter()) {
            paragraph.render(chunk, buf);
        }
    }
}

pub fn keybindings_widget(view: View) -> KeybindingsWidget {
    let (used_groups, keybindings) = binding_for_view(view);

    let mut grouped_keybindings: Vec<Paragraph<'static>> = Vec::new();
    for (i, group) in Group::iter().enumerate() {
        if !used_groups.contains(&group) {
            continue;
        }

        let grouped_lines: Vec<Line<'_>> = keybindings
            .iter()
            .filter(|b| b.group == group)
            .map(|b| Line::from_iter([b.symbol.red().bold(), " - ".into(), b.description.into()]))
            .collect();

        let mut block: Block<'_> = Block::default().padding(Padding::new(1, 1, 0, 0));

        if i != 0 {
            block = block.borders(Borders::LEFT);
        }

        grouped_keybindings.push(
            Paragraph::new(grouped_lines)
                .alignment(Alignment::Center)
                .block(block),
        );
    }

    KeybindingsWidget {
        grouped: grouped_keybindings,
    }
}
