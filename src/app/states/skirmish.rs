use crate::app::{
    states::{FocusedCell, Offset},
    widgets::CellWidget,
};
use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub map_width: usize,
    pub map_height: usize,
    pub vertical_offset: Offset,
    pub horizontal_offset: Offset,
    pub board_cells: Vec<CellWidget>,
    pub zoom_level: ZoomLevel,
    pub focused_cell: FocusedCell,
}

impl SkirmishState {
    pub fn init_board(&mut self) {
        if !self.board_cells.is_empty() {
            return;
        }

        for row in 0..self.map_height {
            for col in 0..self.map_width {
                self.board_cells.push(CellWidget::new(row, col, false));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum ZoomLevel {
    ZoomedIn,
    Default,
    ZoomedOut,
}
