use crate::app::widgets::CellWidget;
use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    value: usize,
    max: usize,
    value_initiated: bool,
    max_initiated: bool,
}

impl Offset {
    pub fn new() -> Self {
        Self {
            value: 0,
            max: 0,
            value_initiated: false,
            max_initiated: false,
        }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_initial_value(&mut self, value: usize) {
        if self.value_initiated {
            return;
        }

        self.value = value;
        self.value_initiated = true;
    }

    pub fn set_max(&mut self, max: usize) {
        // if self.max_initiated {
        //     return;
        // }

        self.max = max;
        self.max_initiated = true;
    }

    pub fn next(&mut self) {
        self.value = self.value.saturating_add(1).min(self.max);
    }

    pub fn prev(&mut self) {
        self.value = self.value.saturating_sub(1).max(0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusedCell {
    pub row: usize,
    pub col: usize,
    pub max_row: usize,
    pub max_col: usize,
}

impl FocusedCell {
    pub fn new(row: usize, col: usize, max_row: usize, max_col: usize) -> Self {
        Self {
            row,
            col,
            max_row,
            max_col,
        }
    }

    pub fn move_up(&mut self) {
        self.row = self.row.saturating_sub(1).max(0);
    }

    pub fn move_down(&mut self) {
        self.row = self.row.saturating_add(1).min(self.max_row - 1);
    }

    pub fn move_left(&mut self) {
        self.col = self.col.saturating_sub(1).max(0);
    }

    pub fn move_right(&mut self) {
        self.col = self.col.saturating_add(1).min(self.max_col - 1);
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
