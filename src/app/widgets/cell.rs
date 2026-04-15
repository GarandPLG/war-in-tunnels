use crate::app::{
    buildings::Buildings,
    states::{CellTag, Players, ZoomLevel},
    units::Units,
};
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
    tag: CellTag,
    marked: bool,
    unit: Option<Units>,
    building: Option<Buildings>,
}

impl CellWidget {
    pub fn new(
        row: usize,
        col: usize,
        zoom_level: ZoomLevel,
        selected: bool,
        tag: CellTag,
        unit: Option<Units>,
        building: Option<Buildings>,
    ) -> Self {
        Self {
            row,
            col,
            selected,
            zoom_level,
            tag,
            marked: false,
            unit,
            building,
        }
    }

    pub fn set_selected(&mut self, selected: bool) -> &mut Self {
        self.selected = selected;
        self
    }

    pub fn set_zoom_level(&mut self, zoom_level: ZoomLevel) {
        self.zoom_level = zoom_level;
    }

    pub fn set_tag(&mut self, tag: CellTag) {
        self.tag = tag;
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
        if self.selected || self.tag != CellTag::Stone || self.marked {
            format!("{}{}", self.col_to_letters(), self.row).green()
        } else {
            "".to_span()
        }
    }

    fn fg_color(&self) -> Color {
        match self.tag {
            _ if self.marked && self.selected => Color::Magenta,
            _ if self.marked && !self.selected => Color::LightMagenta,
            _ if self.selected && !self.marked => Color::LightYellow,
            CellTag::Base(Players::Player) if !self.selected => Color::LightBlue,
            CellTag::Base(Players::Enemy) if !self.selected => Color::LightRed,
            CellTag::Tunel if !self.selected => Color::Gray,
            _ => Color::White,
        }
    }

    fn get_text_area(&self) -> Vec<Line<'_>> {
        let tag: &str = match self.tag {
            CellTag::Base(_) => "B",
            CellTag::Tunel => "T",
            CellTag::Stone => " ",
        };
        let units: &str = " "; // TODO: units count on that cell

        let mut text_area: Vec<Line<'_>> = Vec::new();

        match self.zoom_level {
            ZoomLevel::ZoomedIn => {
                text_area.push(Line::from(format!("          {}", units)));
                text_area.push(Line::from(format!("           ")));
                text_area.push(Line::from(format!("     {}     ", tag)));
                text_area.push(Line::from(format!("           ")));
                text_area.push(Line::from(format!("           ")));
            }
            ZoomLevel::Default => {
                text_area.push(Line::from(format!("      {}", units)));
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
