#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FocusedCell {
    row: usize,
    col: usize,
    max_row: usize,
    max_col: usize,
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

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
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
