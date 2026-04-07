use crate::app::states::ZoomLevel;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::Line,
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
    row: usize,
    col: usize,
    selected: bool,
    zoom_level: ZoomLevel,
    // pub tags: Vec<CellTags>,
}

impl CellWidget {
    pub fn new(row: usize, col: usize, zoom_level: ZoomLevel) -> Self {
        Self {
            row,
            col,
            selected: false,
            zoom_level,
        }
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    pub fn set_zoom_level(&mut self, zoom_level: ZoomLevel) -> &mut Self {
        self.zoom_level = zoom_level;
        self
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

    fn display_coords(&self) -> String {
        format!("{}{}", self.col_to_letters(), self.row)
    }

    fn fg_color(&self) -> Color {
        if self.selected {
            Color::Red
        } else {
            Color::White
        }
    }

    fn get_text_area(&self) -> Vec<Line<'_>> {
        let mut text_area: Vec<Line<'_>> = Vec::new();

        match self.zoom_level {
            ZoomLevel::ZoomedIn => {
                text_area.push(Line::from("           "));
                text_area.push(Line::from("           "));
                text_area.push(Line::from("           "));
                text_area.push(Line::from("           "));
                text_area.push(Line::from("           "));
            }
            ZoomLevel::Default => {
                text_area.push(Line::from("       "));
                text_area.push(Line::from("       "));
                text_area.push(Line::from("       "));
            }
            ZoomLevel::ZoomedOut => {
                text_area.push(Line::from("   "));
            }
        }

        text_area
    }

    fn get_block(&self) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(self.fg_color()))
            .title(self.display_coords().green())
    }
}

impl Widget for CellWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.get_text_area())
            .alignment(Alignment::Center)
            .block(self.get_block())
            .render(area, buf);
    }
}
