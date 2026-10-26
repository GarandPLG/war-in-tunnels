use crate::app::states::skirmish_states::{BoardState, Players, tasks::Task, units::Unit};
use crate::app::widgets::CellWidget;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::Widget,
};
use std::{collections::HashSet, rc::Rc};

pub struct BoardWidget<'a> {
    state: &'a BoardState,
    stress: bool,
}

impl<'a> BoardWidget<'a> {
    pub fn new(state: &'a BoardState, stress: bool) -> Self {
        Self { state, stress }
    }
}

impl Widget for BoardWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut queued_path: HashSet<(usize, usize)> = HashSet::new();
        for row in &self.state.cells {
            for cell in row {
                if let Some(unit) = cell.get_ref_option_unit()
                    && unit.get_owner() == Players::Player
                {
                    for task in unit.get_ref_tasks() {
                        queued_path.extend(task.get_path().iter().copied());
                    }
                }
            }
        }
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
                    .map(|cell: &CellWidget| -> () {
                        let mut cell: CellWidget = cell.clone();
                        let coords: (usize, usize) = (
                            row_idx + self.state.vertical_offset.get_value(),
                            col_idx + self.state.horizontal_offset.get_value(),
                        );
                        if queued_path.contains(&coords)
                            && (coords != self.state.get_focused_cell().get_coords()
                                || self.state.marked_cells.marking_cells)
                        {
                            cell.set_marked(true);
                        }
                        if self.stress {
                            cell.render_stress(*cell_area, buf);
                        } else {
                            cell.render(*cell_area, buf);
                        }
                    });
            }
        }
    }
}
