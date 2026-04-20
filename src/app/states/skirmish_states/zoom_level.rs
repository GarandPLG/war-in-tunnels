use clap::ValueEnum;
use ratatui::text::Line;

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

    pub fn get_cell_text_area(
        &self,
        tag: char,
        structure_level: char,
        unit: char,
    ) -> Vec<Line<'_>> {
        match self {
            ZoomLevel::ZoomedIn => vec![
                Line::from(format!(" {}       {} ", structure_level, unit)),
                Line::from(format!("           ")),
                Line::from(format!("     {}     ", tag)),
                Line::from(format!("           ")),
                Line::from(format!("           ")),
            ],
            ZoomLevel::Default => vec![
                Line::from(format!(" {}   {} ", structure_level, unit)),
                Line::from(format!("   {}   ", tag)),
                Line::from(format!("       ")),
            ],
            ZoomLevel::ZoomedOut => vec![Line::from(format!(" {} ", tag))],
        }
    }
}
