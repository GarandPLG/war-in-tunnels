use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Widget},
};

// pub enum CellTags {
//     Player,
//     Enemy,
//     Dirt,
//     Tunnel,
//     Base,
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellWidget {
    pub row: usize,
    pub col: usize,
    pub selected: bool,
    // pub tags: Vec<CellTags>,
}

impl CellWidget {
    pub fn new(row: usize, col: usize, selected: bool) -> Self {
        Self { row, col, selected }
    }

    fn col_to_letters(&self) -> String {
        let mut col: usize = self.col + 1;
        let mut letters: Vec<char> = Vec::new();

        while col > 0 {
            letters.push((b'A' + ((col - 1) % 26) as u8) as char);
            col = (col - 1) / 26;
        }

        letters.iter().rev().collect()
    }

    pub fn display_coords(&self) -> String {
        format!("{}{}", self.col_to_letters(), self.row)
    }

    fn is_selected(&self) -> Color {
        if self.selected {
            Color::Red
        } else {
            Color::White
        }
    }
}

impl Widget for CellWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("")
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(self.is_selected()))
                    .title(self.display_coords().green()),
            )
            .render(area, buf);
    }
}
