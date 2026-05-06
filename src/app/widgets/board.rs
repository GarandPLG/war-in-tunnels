use crate::app::states::skirmish_states::BoardState;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use std::rc::Rc;

pub struct BoardWidget<'a> {
    state: &'a BoardState<'a>,
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
                self.state
                    .cells
                    .get(row_idx + self.state.vertical_offset.get_value())
                    .and_then(|rows| rows.get(col_idx + self.state.horizontal_offset.get_value()))
                    .map(|cell| cell.clone().render(*cell_area, buf));
            }
        }
    }
}
