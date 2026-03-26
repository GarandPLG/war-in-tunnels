use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

// pub enum CellTags {
//     Player,
//     Enemy,
//     Dirt,
//     Tunnel,
//     Base,
// }

pub struct CellWidget {
    pub row: usize,
    pub col: usize,
    // pub tags: Vec<CellTags>,
}

impl CellWidget {
    pub fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

impl Widget for CellWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!("{}/{}", self.row, self.col))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).white())
            .render(area, buf);
    }
}
