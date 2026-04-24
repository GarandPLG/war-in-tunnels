use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Color,
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

use crate::app::helpers::block_title_helper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SidePanelWidget {
    coords: (usize, usize),
    structure_name: &'static str,
    unit_name: &'static str,
}

impl SidePanelWidget {
    pub fn new(
        coords: (usize, usize),
        structure_name: &'static str,
        unit_name: &'static str,
    ) -> Self {
        Self {
            coords,
            structure_name,
            unit_name,
        }
    }

    fn col_to_letters(&self) -> String {
        let mut col: usize = self.coords.1 + 1;
        let mut letters: Vec<char> = Vec::new();

        while col > 0 {
            letters.push((b'A' + ((col - 1) % 26) as u8) as char);
            col = (col - 1) / 26;
        }

        letters.iter().rev().collect()
    }

    fn get_title(&self) -> Line<'_> {
        let cell_coords: String = format!("{}{}", self.col_to_letters(), self.coords.0);

        let mut texts: Vec<(String, Color)> = Vec::with_capacity(3);

        texts.push((cell_coords, Color::Yellow));
        texts.push((self.structure_name.to_string(), Color::Cyan));

        if !self.unit_name.is_empty() {
            texts.push((self.unit_name.to_string(), Color::Green));
        }

        block_title_helper(&texts, Some(" - "))
    }

    fn get_block(&self) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .title(self.get_title())
            .title_alignment(Alignment::Center)
    }
}

impl Widget for SidePanelWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::default()
            .alignment(Alignment::Center)
            .block(self.get_block())
            .render(area, buf);
    }
}
