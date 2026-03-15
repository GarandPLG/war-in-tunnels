use crate::app::keybindings::{Action, Group, KeyBinding, binding_for};
use clap::ValueEnum;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::collections::HashSet;

pub struct KeybindingsWidget {
    grouped: Vec<Paragraph<'static>>,
}

impl KeybindingsWidget {
    pub fn new(actions: Vec<Action>) -> Self {
        let keybindings: Vec<Option<&'static KeyBinding>> =
            actions.iter().map(|a| binding_for(*a)).collect();

        let used_groups: HashSet<Group> = keybindings
            .iter()
            .filter_map(|b| b.map(|binding| binding.group))
            .collect();

        let mut grouped_keybindings: Vec<Paragraph<'static>> = Vec::new();
        for (i, group) in Group::value_variants().iter().enumerate() {
            if !used_groups.contains(group) {
                continue;
            }

            let grouped_lines: Vec<Line<'_>> = keybindings
                .iter()
                .filter(|b| b.as_ref().map_or(false, |kb| kb.group == *group))
                .filter_map(|b| {
                    b.as_ref().map(|kb| {
                        Line::from_iter([
                            kb.symbol.red().bold(),
                            " - ".into(),
                            kb.description.into(),
                        ])
                    })
                })
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

        Self {
            grouped: grouped_keybindings,
        }
    }
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
