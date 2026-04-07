pub enum MoveFocusedCell {
    Up,
    Down,
    Left,
    Right,
}

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

    pub fn move_focused_cell(&mut self, direction: MoveFocusedCell) -> (usize, usize) {
        match direction {
            MoveFocusedCell::Up => {
                self.row = self.row.saturating_sub(1).max(0);
            }
            MoveFocusedCell::Down => {
                self.row = self.row.saturating_add(1).min(self.max_row - 1);
            }
            MoveFocusedCell::Left => {
                self.col = self.col.saturating_sub(1).max(0);
            }
            MoveFocusedCell::Right => {
                self.col = self.col.saturating_add(1).min(self.max_col - 1);
            }
        }

        (self.row, self.col)
    }
}
