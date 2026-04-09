use crate::app::{
    helpers::{CellSizes, cell_size_helper, cells_area_helper},
    states::{FocusedCell, Offset, ZoomLevel, skirmish_states::MoveFocusedCell},
    widgets::{CellTag, CellWidget, Players},
};
use ratatui::layout::Rect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardState {
    cells_area: Rect,
    pub cell_width: usize,
    pub cell_height: usize,
    pub map_width: usize,
    pub map_height: usize,
    pub cols: usize,
    pub rows: usize,
    pub vertical_offset: Offset,
    pub horizontal_offset: Offset,
    pub cells: Vec<Vec<CellWidget>>,
    pub zoom_level: ZoomLevel,
    focused_cell: FocusedCell,
    player_base_coords: (usize, usize),
    enemy_base_coords: (usize, usize),
}

impl BoardState {
    pub fn new(area: &Rect, map_width: usize, map_height: usize, zoom_level: ZoomLevel) -> Self {
        let cells_area: Rect = cells_area_helper(area);

        let cell_width: usize = cell_size_helper(CellSizes::Width, zoom_level);
        let cell_height: usize = cell_size_helper(CellSizes::Height, zoom_level);

        let cols: usize = (cells_area.width / cell_width as u16) as usize;
        let rows: usize = (cells_area.height / cell_height as u16) as usize;

        let v_max_offset: usize = Self::max_offset(map_height, rows);
        let h_max_offset: usize = Self::max_offset(map_width, cols);

        let vertical_offset: Offset = Offset::new(
            Some((map_height - rows as usize + 1) / 2),
            Some(v_max_offset),
        );
        let horizontal_offset: Offset = Offset::new(None, Some(h_max_offset));

        let focused_cell: FocusedCell =
            FocusedCell::new((map_height) / 2, 2, map_height, map_width);

        let mut cells: Vec<Vec<CellWidget>> = Vec::new();

        let player_base_coords: (usize, usize) = ((map_height) / 2, 1);
        let enemy_base_coords: (usize, usize) = ((map_height) / 2, map_width - 2);

        for row in 0..map_height {
            let mut rows: Vec<CellWidget> = Vec::new();

            for col in 0..map_width {
                let selected: bool = row == focused_cell.get_row() && col == focused_cell.get_col();

                let tag: CellTag = if row == player_base_coords.0 && col == player_base_coords.1 {
                    CellTag::Base(Players::Player)
                } else if row == enemy_base_coords.0 && col == enemy_base_coords.1 {
                    CellTag::Base(Players::Enemy)
                } else {
                    CellTag::Stone
                };

                rows.push(CellWidget::new(row, col, zoom_level, selected, tag));
            }

            cells.push(rows);
        }

        Self {
            cells_area,
            cell_width,
            cell_height,
            map_width,
            map_height,
            cols,
            rows,
            vertical_offset,
            horizontal_offset,
            cells,
            zoom_level,
            focused_cell,
            player_base_coords,
            enemy_base_coords,
        }
    }

    fn get_mut_cell(&mut self, row: usize, col: usize) -> &mut CellWidget {
        &mut self.cells[row][col]
    }

    fn max_offset(map_size: usize, size: usize) -> usize {
        if map_size > size { map_size - size } else { 0 }
    }

    pub fn resize_change(&mut self, area: &Rect) {
        self.cells_area = cells_area_helper(area);

        self.cols = (self.cells_area.width / self.cell_width as u16) as usize;
        self.rows = (self.cells_area.height / self.cell_height as u16) as usize;

        let h_max_offset: usize = Self::max_offset(self.map_height, self.cols);
        let v_max_offset: usize = Self::max_offset(self.map_width, self.rows);

        self.horizontal_offset =
            Offset::new(Some(self.horizontal_offset.get_value()), Some(h_max_offset));
        self.vertical_offset =
            Offset::new(Some(self.vertical_offset.get_value()), Some(v_max_offset));
    }

    pub fn zoom_change(&mut self, new_zoom_level: ZoomLevel) {
        self.zoom_level = new_zoom_level;

        self.cell_width = cell_size_helper(CellSizes::Width, self.zoom_level);
        self.cell_height = cell_size_helper(CellSizes::Height, self.zoom_level);

        self.cols = (self.cells_area.width / self.cell_width as u16) as usize;
        self.rows = (self.cells_area.height / self.cell_height as u16) as usize;

        let v_max_offset: usize = Self::max_offset(self.map_height, self.rows);
        let h_max_offset: usize = Self::max_offset(self.map_width, self.cols);

        self.vertical_offset =
            Offset::new(Some(self.vertical_offset.get_value()), Some(v_max_offset));
        self.horizontal_offset =
            Offset::new(Some(self.horizontal_offset.get_value()), Some(h_max_offset));

        self.focused_cell = FocusedCell::new(
            self.focused_cell.get_row(),
            self.focused_cell.get_col(),
            self.map_height,
            self.map_width,
        );

        for row in 0..self.map_height {
            for col in 0..self.map_width {
                self.get_mut_cell(row, col).set_zoom_level(new_zoom_level);
            }
        }
    }

    pub fn change_focused_cell(&mut self, direction: MoveFocusedCell) {
        let old_row: usize = self.focused_cell.get_row();
        let old_col: usize = self.focused_cell.get_col();
        let (new_row, new_col) = self.focused_cell.move_focused_cell(direction);

        self.get_mut_cell(old_row, old_col).set_selected(false);
        self.get_mut_cell(new_row, new_col).set_selected(true);
    }

    pub fn is_focused_cell_visible(&self) -> bool {
        let vertical_offset: usize = self.vertical_offset.get_value();
        let horizontal_offset: usize = self.horizontal_offset.get_value();

        let row: usize = self.focused_cell.get_row();
        let col: usize = self.focused_cell.get_col();

        if 0usize.saturating_add(vertical_offset) > row
            || row >= self.rows.saturating_add(vertical_offset)
        {
            return false;
        }
        if 0usize.saturating_add(horizontal_offset) > col
            || col >= self.cols.saturating_add(horizontal_offset)
        {
            return false;
        }

        true
    }
}
