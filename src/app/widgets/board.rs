use crate::app::{App, widgets::CellWidget};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use std::rc::Rc;

pub struct BoardWidget {
    cell_width: u16,
    cell_height: u16,
    cols: u16,
    rows: u16,
    h_offset: usize,
    v_offset: usize,
}

impl BoardWidget {
    pub fn new(app: &App, area_width: u16, area_height: u16) -> Self {
        let cell_width: u16 = 7;
        let cell_height: u16 = 4;

        let cols: u16 = area_width / cell_width;
        let rows: u16 = area_height / cell_height;

        let h_max_offset: u16 = if app.states.settings.map_height as u16 > rows {
            app.states.settings.map_height as u16 - rows
        } else {
            0
        };

        let v_max_offset: u16 = if app.states.settings.map_width as u16 > cols {
            app.states.settings.map_width as u16 - cols
        } else {
            0
        };

        // let h_offset: usize = (cell_height * rows as u16 / area_height) as usize;
        // let v_offset: usize = (cell_width * cols as u16 / area_width) as usize;

        Self {
            cell_width,
            cell_height,
            cols,
            rows,
            h_offset: h_max_offset as usize,
            v_offset: v_max_offset as usize,
        }
    }
}

impl Widget for BoardWidget {
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
                let cell: CellWidget =
                    CellWidget::new(row_idx + self.v_offset, col_idx + self.h_offset);

                cell.render(*cell_area, buf);
            }
        }
    }
}
