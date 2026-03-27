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
    pub fn new(app: &mut App, area_width: u16, area_height: u16) -> Self {
        const CELL_HIGHT: u16 = 8; // 4
        const CELL_WIDTH: u16 = 14; // 7

        let rows: u16 = area_height / CELL_HIGHT;
        let cols: u16 = area_width / CELL_WIDTH;

        let v_max_offset: usize = if app.states.settings.map_height as u16 > rows {
            app.states.settings.map_height as u16 - rows
        } else {
            0
        } as usize;

        let h_max_offset: usize = if app.states.settings.map_width as u16 > cols {
            app.states.settings.map_width as u16 - cols
        } else {
            0
        } as usize;

        app.states.skirmish.horizontal_offset.set_max(h_max_offset);
        app.states.skirmish.vertical_offset.set_max(v_max_offset);

        Self {
            cell_width: CELL_WIDTH,
            cell_height: CELL_HIGHT,
            cols,
            rows,
            h_offset: app.states.skirmish.horizontal_offset.get_value(),
            v_offset: app.states.skirmish.vertical_offset.get_value(),
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
