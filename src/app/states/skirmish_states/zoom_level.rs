use clap::ValueEnum;
use ratatui::text::{Line, Span};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum ZoomLevel {
    ZoomedIn,
    Default,
    ZoomedOut,
}

pub enum CellSizes {
    Width,
    Height,
}

impl ZoomLevel {
    pub fn get_cell_size(&self, cell_size: CellSizes) -> usize {
        match (cell_size, self) {
            (CellSizes::Width, ZoomLevel::ZoomedIn) => 13,
            (CellSizes::Width, ZoomLevel::Default) => 9,
            (CellSizes::Width, ZoomLevel::ZoomedOut) => 5,
            (CellSizes::Height, ZoomLevel::ZoomedIn) => 7,
            (CellSizes::Height, ZoomLevel::Default) => 5,
            (CellSizes::Height, ZoomLevel::ZoomedOut) => 3,
        }
    }

    pub fn get_cell_text_area<'a>(
        &self,
        tag: char,
        structure_level: char,
        unit: Span<'a>,
    ) -> Vec<Line<'a>> {
        match self {
            ZoomLevel::ZoomedIn => vec![
                Line::from(vec![
                    Span::raw(format!(" {}       ", structure_level)),
                    unit,
                    Span::raw(" "),
                ]),
                Line::from(format!("           ")),
                Line::from(format!("     {}     ", tag)),
                Line::from(format!("           ")),
                Line::from(format!("           ")),
            ],
            ZoomLevel::Default => vec![
                Line::from(vec![
                    Span::raw(format!(" {}   ", structure_level)),
                    unit,
                    Span::raw(" "),
                ]),
                Line::from(format!("   {}   ", tag)),
                Line::from(format!("       ")),
            ],
            ZoomLevel::ZoomedOut => vec![Line::from(format!(" {} ", tag))],
        }
    }
}
