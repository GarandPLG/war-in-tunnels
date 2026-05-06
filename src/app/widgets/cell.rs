use crate::app::states::skirmish_states::{
    ZoomLevel,
    structures::{Stone, Structure, Structures},
    units::{Unit, Units},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, ToSpan},
    widgets::{Block, BorderType, Borders, Paragraph, Widget},
};

#[derive(Debug, Clone, PartialEq)]
pub struct CellWidget {
    row: usize,
    col: usize,
    selected: bool,
    zoom_level: ZoomLevel,
    structure: Structures,
    marked: bool,
    unit: Option<Units>,
}

impl CellWidget {
    pub fn new(
        row: usize,
        col: usize,
        zoom_level: ZoomLevel,
        selected: bool,
        structure: Structures,
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

    pub fn get_structure(&self) -> Structures {
        self.structure
    }

    pub fn get_option_unit(&self) -> Option<Units> {
        self.unit.clone()
    }

    pub fn set_structure(&mut self, structure: Structures) -> &mut Self {
        self.structure = structure;
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
        if self.selected || self.structure != Structures::Stone(Stone::new()) || self.marked {
            format!("{}{}", self.col_to_letters(), self.row).green()
        } else {
            "".to_span()
        }
    }

    fn fg_color(&self) -> Color {
        if self.marked && self.selected {
            Color::Magenta
        } else if self.marked && !self.selected {
            Color::LightMagenta
        } else if !self.marked && self.selected {
            Color::LightYellow
        } else {
            self.structure.get_color()
        }
    }

    fn get_text_area(&self) -> Vec<Line<'_>> {
        self.zoom_level.get_cell_text_area(
            self.structure.get_tag(),
            self.structure.get_level(),
            self.unit.get_tag(),
        )
    }

    fn get_block(&self) -> Block<'_> {
        Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(self.fg_color()))
            .title(self.display_coords())
            .border_type(BorderType::Rounded)
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
