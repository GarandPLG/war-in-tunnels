use crate::app::{App, states::ZoomLevel, widgets::CellWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use std::rc::Rc;

pub struct BoardWidget<'a> {
    map_width: usize,
    cell_width: u16,
    cell_height: u16,
    cols: u16,
    rows: u16,
    h_offset: usize,
    v_offset: usize,
    cells: &'a Vec<CellWidget>,
}

impl<'a> BoardWidget<'a> {
    fn max_offset(map_size: u16, size: u16) -> usize {
        if map_size > size {
            (map_size - size) as usize
        } else {
            0
        }
    }

    pub fn new(app: &'a mut App, area_width: u16, area_height: u16) -> Self {
        let cell_height: u16 = match app.states.skirmish.zoom_level {
            ZoomLevel::ZoomedIn => 7,
            ZoomLevel::Default => 5,
            ZoomLevel::ZoomedOut => 3,
        };

        let cell_width: u16 = match app.states.skirmish.zoom_level {
            ZoomLevel::ZoomedIn => 13,
            ZoomLevel::Default => 9,
            ZoomLevel::ZoomedOut => 5,
        };

        let rows: u16 = area_height / cell_height;
        let cols: u16 = area_width / cell_width;

        let v_max_offset: usize = Self::max_offset(app.states.skirmish.map_height as u16, rows);
        let h_max_offset: usize = Self::max_offset(app.states.skirmish.map_width as u16, cols);

        app.states.skirmish.horizontal_offset.set_max(h_max_offset);
        app.states.skirmish.vertical_offset.set_max(v_max_offset);

        if v_max_offset > 0 {
            app.states
                .skirmish
                .vertical_offset
                .set_initial_value((app.states.skirmish.map_height - rows as usize) / 2);
        }

        Self {
            map_width: app.states.skirmish.map_width as usize,
            cell_width,
            cell_height,
            cols,
            rows,
            h_offset: app.states.skirmish.horizontal_offset.get_value(),
            v_offset: app.states.skirmish.vertical_offset.get_value(),
            cells: &app.states.skirmish.board_cells,
        }
    }
}

impl Widget for BoardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal: Rc<[Rect]> = Layout::horizontal(vec![
            Constraint::Length(self.cell_width);
            self.cols as usize
        ])
        .split(area);

        for (col_idx, col_area) in horizontal.iter().enumerate() {
            let vertical: Rc<[Rect]> = Layout::vertical(vec![
                Constraint::Length(self.cell_height);
                self.rows as usize
            ])
            .split(*col_area);

            for (row_idx, cell_area) in vertical.iter().enumerate() {
                if let Some(cell) = self
                    .cells
                    .get((row_idx + self.v_offset) * self.map_width + (col_idx + self.h_offset))
                {
                    cell.render(*cell_area, buf);
                }
            }
        }
    }
}
