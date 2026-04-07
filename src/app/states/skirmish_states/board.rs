use crate::app::{
    helpers::{CellSizes, cell_size_helper, cells_area_helper},
    states::{FocusedCell, Offset, ZoomLevel},
    widgets::CellWidget,
};
use ratatui::layout::Rect;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoardState {
    pub cells_area: Rect,
    pub cell_width: usize,
    pub cell_height: usize,
    pub map_width: usize,
    pub map_height: usize,
    pub cols: usize,
    pub rows: usize,
    pub vertical_offset: Offset,
    pub horizontal_offset: Offset,
    pub cells: Vec<CellWidget>,
    pub zoom_level: ZoomLevel,
    pub focused_cell: FocusedCell,
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

        let vertical_offset: Offset =
            Offset::new(Some((map_height - rows as usize) / 2), Some(v_max_offset));
        let horizontal_offset: Offset = Offset::new(None, Some(h_max_offset));

        let focused_cell: FocusedCell =
            FocusedCell::new((map_height / 2) - 1, 2, map_height, map_width);

        let mut cells: Vec<CellWidget> = Vec::new();

        for row in 0..map_height {
            for col in 0..map_width {
                cells.push(CellWidget::new(row, col));
            }
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
        }
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
    }

    fn max_offset(map_size: usize, size: usize) -> usize {
        if map_size > size { map_size - size } else { 0 }
    }
}
