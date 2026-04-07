use crate::app::states::skirmish_states::BoardState;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use std::rc::Rc;

pub struct BoardWidget<'a> {
    state: &'a BoardState,
}

impl<'a> BoardWidget<'a> {
    pub fn new(state: &'a BoardState) -> Self {
        Self { state }
    }
}

impl Widget for BoardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let horizontal: Rc<[Rect]> =
            Layout::horizontal(vec![
                Constraint::Length(self.state.cell_width as u16);
                self.state.cols
            ])
            .split(area);

        for (col_idx, col_area) in horizontal.iter().enumerate() {
            let vertical: Rc<[Rect]> =
                Layout::vertical(vec![
                    Constraint::Length(self.state.cell_height as u16);
                    self.state.rows
                ])
                .split(*col_area);

            for (row_idx, cell_area) in vertical.iter().enumerate() {
                let map_row: usize = row_idx + self.state.vertical_offset.get_value();
                let map_col: usize = col_idx + self.state.horizontal_offset.get_value();

                if let Some(cell) = self
                    .state
                    .cells
                    .get(map_row * self.state.map_width + map_col)
                {
                    cell.render(*cell_area, buf);
                }
            }
        }
    }
}
