use crate::app::states::{CellStructure, Units, ZoomLevel, skirmish_states::buildings::Stone};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, Borders, Paragraph, Widget},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellWidget {
    row: usize,
    col: usize,
    selected: bool,
    zoom_level: ZoomLevel,
    structure: CellStructure,
    marked: bool,
    unit: Option<Units>,
}

impl CellWidget {
    pub fn new(
        row: usize,
        col: usize,
        zoom_level: ZoomLevel,
        selected: bool,
        structure: CellStructure,
        unit: Option<Units>,
    ) -> Self {
        Self {
            row,
            col,
            selected,
            zoom_level,
            marked: false,
            structure,
            unit,
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

    pub fn set_structure(&mut self, sctructure: CellStructure) -> &mut Self {
        self.structure = sctructure;
        self
    }

    pub fn get_marked(&self) -> bool {
        self.marked
    }

    pub fn set_marked(&mut self, marked: bool) -> &mut Self {
        self.marked = marked;
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

    fn display_coords(&self) -> Span<'_> {
        if self.selected || self.structure != CellStructure::Stone(Stone::new()) || self.marked {
            format!("{}{}", self.col_to_letters(), self.row).green()
        } else {
            "".to_span()
        }
    }

    fn fg_color(&self) -> Color {
        if self.marked && self.selected {
            return Color::Magenta;
        } else if self.marked && !self.selected {
            return Color::LightMagenta;
        } else if self.selected && !self.marked {
            return Color::LightYellow;
        }

        if !self.selected {
            return match self.structure {
                CellStructure::Base(base) => base.get_color(),
                CellStructure::Tunnel(tunnel) => tunnel.get_color(),
                CellStructure::Stone(stone) => stone.get_color(),
            };
        }

        Color::White
    }

    fn get_text_area(&self) -> Vec<Line<'_>> {
        let tag: char = match self.structure {
            CellStructure::Base(base) => base.get_tag(),
            CellStructure::Tunnel(tunnel) => tunnel.get_tag(),
            CellStructure::Stone(stone) => stone.get_tag(),
        };

        let b_level: char = match self.structure {
            CellStructure::Base(base) => base.get_level(),
            _ => ' ',
        };

        let unit: char = match self.unit {
            Some(unit) => match unit {
                Units::Miner(miner) => miner.get_tag(),
                // _ => ' ',
            },
            None => ' ',
        };

        let mut text_area: Vec<Line<'_>> = Vec::new();

        match self.zoom_level {
            ZoomLevel::ZoomedIn => {
                text_area.push(Line::from(format!(" {}       {} ", b_level, unit)));
                text_area.push(Line::from(format!("           ")));
                text_area.push(Line::from(format!("     {}     ", tag)));
                text_area.push(Line::from(format!("           ")));
                text_area.push(Line::from(format!("           ")));
            }
            ZoomLevel::Default => {
                text_area.push(Line::from(format!(" {}   {} ", b_level, unit)));
                text_area.push(Line::from(format!("   {}   ", tag)));
                text_area.push(Line::from(format!("       ")));
            }
            ZoomLevel::ZoomedOut => {
                text_area.push(Line::from(format!(" {} ", tag)));
            }
        }

        text_area
    }

    fn get_block(&self) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(self.fg_color()))
            .title(self.display_coords())
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
