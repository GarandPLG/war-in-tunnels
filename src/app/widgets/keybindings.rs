use crate::app::{
    helpers::block_single_title_helper,
    keybindings::{Action, Group, KeyBinding, binding_for},
};
use clap::ValueEnum;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use std::collections::HashSet;

/// A widget that visualises the keybindings for a set of actions.
///
/// The widget stores a vector of [`Paragraph`] objects, each representing one
/// logical group of keybindings (e.g. navigation, editing, etc.).  The paragraphs
/// are rendered side‑by‑side inside a bordered container.
pub struct KeybindingsWidget {
    /// Paragraphs, one per used `Group`, containing the formatted keybinding lines.
    grouped: Vec<Paragraph<'static>>,
}

impl KeybindingsWidget {
    /// Create a new widget from a list of actions.
    ///
    /// For each action, the corresponding static [`KeyBinding`] is looked up
    /// using [`binding_for`].  Only groups that have at least one binding are
    /// displayed.  The resulting widget contains a paragraph for each group,
    /// formatted with the key symbol (red & bold) followed by its description.
    ///
    /// # Parameters
    ///
    /// * `actions` – A slice of actions for which keybindings should be shown.
    ///
    /// # Returns
    ///
    /// A fully‑initialised `KeybindingsWidget` ready for rendering.
    pub fn new(actions: &[Action]) -> Self {
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
    /// Render the widget inside the given rectangular area.
    ///
    /// The widget first draws a surrounding border with the title *Keybindings*.
    /// It then divides the inner area into equal horizontal slices—one per
    /// used group—and renders each group's paragraph into its slice.
    ///
    /// If no groups are present, the function returns early without drawing
    /// anything.
    fn render(self, area: Rect, buf: &mut Buffer) {
        let count: u16 = self.grouped.len() as u16;

        if count == 0 {
            return;
        }

        let block: Block<'_> = Block::default()
            .borders(Borders::ALL)
            .title(block_single_title_helper("Keybindings", Color::Magenta));

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
