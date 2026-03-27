use crate::app::widgets::CellWidget;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    value: usize,
    max: usize,
}

impl Offset {
    pub fn new() -> Self {
        Self { value: 0, max: 0 }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_max(&mut self, max: usize) {
        if self.max != 0 {
            return;
        }

        self.max = max;
    }

    pub fn next(&mut self) {
        self.value = self.value.saturating_add(1).min(self.max);
    }

    pub fn prev(&mut self) {
        self.value = self.value.saturating_sub(1).max(0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub map_width: usize,
    pub map_height: usize,
    pub vertical_offset: Offset,
    pub horizontal_offset: Offset,
    pub board_cells: Vec<CellWidget>,
}

impl SkirmishState {
    pub fn init_board(&mut self) {
        if !self.board_cells.is_empty() {
            return;
        }

        for row in 0..self.map_height {
            for col in 0..self.map_width {
                self.board_cells.push(CellWidget::new(row, col));
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}
