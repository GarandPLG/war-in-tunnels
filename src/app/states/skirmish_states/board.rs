use crate::app::{
    helpers::cells_area_helper,
    states::{
        FocusedCell, Offset,
        skirmish_states::{
            CellSizes, MarkedCells, MoveFocusedCell, Players, ZoomLevel,
            structures::{BaseBuilding, Ore, Stone, Structures, Tunnel},
            tasks::{DiggingTask, Task, Tasks},
            units::{MinerUnit, Unit, Units},
        },
    },
    widgets::CellWidget,
};
use ratatui::layout::Rect;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
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
    pub marked_cells: MarkedCells,
}

impl BoardState {
    pub fn new(
        area: &Rect,
        map_width: usize,
        map_height: usize,
        zoom_level: ZoomLevel,
        side_panel: bool,
    ) -> Self {
        let cells_area: Rect = cells_area_helper(area, side_panel);

        let cell_width: usize = zoom_level.get_cell_size(CellSizes::Width);
        let cell_height: usize = zoom_level.get_cell_size(CellSizes::Height);

        let cols: usize = (cells_area.width / cell_width as u16) as usize;
        let rows: usize = (cells_area.height / cell_height as u16) as usize;

        let v_max_offset: usize = Self::max_offset(map_height, rows);
        let h_max_offset: usize = Self::max_offset(map_width, cols);

        let vertical_offset: Offset = Offset::new(
            Some(map_height.saturating_sub(rows).div_ceil(2)),
            Some(v_max_offset),
        );
        let horizontal_offset: Offset = Offset::new(None, Some(h_max_offset));

        let focused_cell: FocusedCell =
            FocusedCell::new((map_height) / 2, 3, map_height, map_width);

        let mut cells: Vec<Vec<CellWidget>> = Vec::new();

        let player_base_coords: (usize, usize) = ((map_height) / 2, 2);
        let enemy_base_coords: (usize, usize) = ((map_height) / 2, map_width - 3);

        for row in 0..map_height {
            let mut rows: Vec<CellWidget> = Vec::new();

            for col in 0..map_width {
                let (focused_row, focused_col) = focused_cell.get_coords();
                let selected: bool = row == focused_row && col == focused_col;
                let player_base: bool = row == player_base_coords.0 && col == player_base_coords.1;
                let player_ore: bool =
                    row == player_base_coords.0 && col == player_base_coords.1 - 1;
                let enemy_base: bool = row == enemy_base_coords.0 && col == enemy_base_coords.1;
                let enemy_ore: bool = row == enemy_base_coords.0 && col == enemy_base_coords.1 + 1;

                let structure: Structures = if player_base {
                    Structures::Base(BaseBuilding::new(Players::Player))
                } else if enemy_base {
                    Structures::Base(BaseBuilding::new(Players::Enemy))
                } else if player_ore {
                    Structures::Ore(Ore::new(Players::Player, 1))
                } else if enemy_ore {
                    Structures::Ore(Ore::new(Players::Enemy, 1))
                } else {
                    Structures::Stone(Stone::new())
                };

                let unit: Option<Units> = if player_base {
                    Some(Units::Miner(MinerUnit::new(
                        Players::Player,
                        player_base_coords,
                    )))
                } else if enemy_base {
                    Some(Units::Miner(MinerUnit::new(
                        Players::Enemy,
                        enemy_base_coords,
                    )))
                } else {
                    None
                };

                rows.push(CellWidget::new(
                    row, col, zoom_level, selected, structure, unit,
                ));
            }

            cells.push(rows);
        }

        let marked_cells: MarkedCells = MarkedCells::new();

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
            marked_cells,
        }
    }

    pub fn get_ref_cell(&self, row: usize, col: usize) -> &CellWidget {
        &self.cells[row][col]
    }

    fn get_mut_cell(&mut self, row: usize, col: usize) -> &mut CellWidget {
        &mut self.cells[row][col]
    }

    pub fn get_marked_cells_widgets(&self) -> VecDeque<&CellWidget> {
        self.marked_cells
            .marked_cells
            .iter()
            .map(|&(row, col)| &self.cells[row][col])
            .collect()
    }

    pub fn get_marked_cells(&self) -> &VecDeque<(usize, usize)> {
        &self.marked_cells.marked_cells
    }

    pub fn toggle_marking(&mut self) {
        self.marked_cells.marking_cells = !self.marked_cells.marking_cells;

        if self.marked_cells.marking_cells {
            self.start_marking_cells();
        } else {
            self.end_marking_cells(false);
        }
    }

    pub fn push_marked_cell(&mut self, new_cell: (usize, usize)) {
        let cell: &mut CellWidget = self.get_mut_cell(new_cell.0, new_cell.1);

        if !cell.get_marked() {
            cell.set_marked(true);
        }

        self.marked_cells
            .marked_cells
            .push_back((new_cell.0, new_cell.1));
    }

    pub fn pop_marked_cell(&mut self) {
        if self.marked_cells.marked_cells.len() < 2 {
            return;
        }

        let old: (usize, usize) =
            self.marked_cells.marked_cells[self.marked_cells.marked_cells.len() - 1];
        let new: (usize, usize) =
            self.marked_cells.marked_cells[self.marked_cells.marked_cells.len() - 2];
        let old_is_unique: bool = self
            .marked_cells
            .marked_cells
            .iter()
            .filter(|x| **x == old)
            .count()
            == 1;

        let old_cell: &mut CellWidget = self.get_mut_cell(old.0, old.1).set_selected(false);
        if old_is_unique {
            old_cell.set_marked(false);
        }

        self.get_mut_cell(new.0, new.1).set_selected(true);

        self.marked_cells.marked_cells.pop_back();

        self.focused_cell.set_focused_cell(new);
    }

    pub fn start_marking_cells(&mut self) {
        let (row, col) = self.focused_cell.get_coords();

        let cell: &mut CellWidget = self.get_mut_cell(row, col);
        cell.set_marked(true);

        self.marked_cells.selected_unit = cell.get_option_unit();
        self.marked_cells.marked_cells.push_back((row, col));
    }

    pub fn end_marking_cells(&mut self, del: bool) {
        // let todo_taks_tyle = Tasks::Digging(DiggingTask::);
        for (row, col) in self.marked_cells.marked_cells.clone() {
            self.get_mut_cell(row, col).set_marked(false);
        }

        let (row, col) = self.marked_cells.selected_unit.get_coords();

        if self.marked_cells.marked_cells.len() > 1 && !del {
            let path: VecDeque<(usize, usize)> = self
                .marked_cells
                .marked_cells
                .iter()
                .skip(1)
                .cloned()
                .collect();

            let tick_increase: f32 = self.marked_cells.selected_unit.get_digging_power();

            let task: Tasks = Tasks::Digging(DiggingTask::new(path, tick_increase));

            self.get_mut_cell(row, col)
                .get_mut_option_unit()
                .set_task(task);
        }

        self.marked_cells.selected_unit = None;
        self.marked_cells.marked_cells.clear();
    }

    fn max_offset(map_size: usize, size: usize) -> usize {
        if map_size > size { map_size - size } else { 0 }
    }

    pub fn get_focused_cell(&self) -> &FocusedCell {
        &self.focused_cell
    }

    pub fn is_focused_cell_visible(&self) -> bool {
        let vertical_offset: usize = self.vertical_offset.get_value();
        let horizontal_offset: usize = self.horizontal_offset.get_value();

        let (row, col) = self.focused_cell.get_coords();

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

    pub fn change_resize(&mut self, area: &Rect, side_panel: bool) {
        self.cells_area = cells_area_helper(area, side_panel);

        self.cols = (self.cells_area.width / self.cell_width as u16) as usize;
        self.rows = (self.cells_area.height / self.cell_height as u16) as usize;

        let v_max_offset: usize = Self::max_offset(self.map_height, self.rows);
        let h_max_offset: usize = Self::max_offset(self.map_width, self.cols);

        self.horizontal_offset =
            Offset::new(Some(self.horizontal_offset.get_value()), Some(h_max_offset));
        self.vertical_offset =
            Offset::new(Some(self.vertical_offset.get_value()), Some(v_max_offset));
    }

    pub fn change_zoom(&mut self, new_zoom_level: ZoomLevel) {
        self.zoom_level = new_zoom_level;

        self.cell_width = new_zoom_level.get_cell_size(CellSizes::Width);
        self.cell_height = new_zoom_level.get_cell_size(CellSizes::Height);

        self.cols = (self.cells_area.width / self.cell_width as u16) as usize;
        self.rows = (self.cells_area.height / self.cell_height as u16) as usize;

        let v_max_offset: usize = Self::max_offset(self.map_height, self.rows);
        let h_max_offset: usize = Self::max_offset(self.map_width, self.cols);

        self.vertical_offset =
            Offset::new(Some(self.vertical_offset.get_value()), Some(v_max_offset));
        self.horizontal_offset =
            Offset::new(Some(self.horizontal_offset.get_value()), Some(h_max_offset));

        let (focused_row, focused_coll) = self.focused_cell.get_coords();

        self.focused_cell =
            FocusedCell::new(focused_row, focused_coll, self.map_height, self.map_width);

        for row in 0..self.map_height {
            for col in 0..self.map_width {
                self.get_mut_cell(row, col).set_zoom_level(new_zoom_level);
            }
        }
    }

    pub fn change_focused_cell(&mut self, direction: MoveFocusedCell) {
        let old_cell: (usize, usize) = self.focused_cell.get_coords();
        let new_cell: (usize, usize) = self.focused_cell.move_focused_cell(direction);

        self.get_mut_cell(old_cell.0, old_cell.1)
            .set_selected(false);
        self.get_mut_cell(new_cell.0, new_cell.1).set_selected(true);

        if self.marked_cells.marking_cells {
            self.push_marked_cell(new_cell);
        }
    }

    // fn get_active_tasks(&self) -> Vec<&Tasks> {
    //     self.cells
    //         .iter()
    //         .flat_map(|row| row.iter())
    //         .filter_map(|cell| {
    //             cell.get_ref_option_unit()
    //                 .as_ref()
    //                 .and_then(|unit| unit.get_ref_tasks().front())
    //         })
    //         .collect()
    //     // for row in self.marked_cells.marked_cells {

    //     // }
    // }

    pub fn advance_turn(&mut self) {
        for row in 0..self.map_height {
            for col in 0..self.map_width {
                /* ---------------------------------------------------------
                 * 1️⃣  Take the unit out of the current cell (if any).  This
                 *    gives us an *owned* `Units`, so we no longer hold a
                 *    mutable reference into `self`.
                 * ------------------------------------------------------- */
                let maybe_unit: Option<Units> =
                    self.get_mut_cell(row, col).get_mut_option_unit().take();

                let Some(mut unit) = maybe_unit else {
                    continue; // no unit in this cell
                };

                /* ---------------------------------------------------------
                 * 2️⃣  Look at the unit’s task queue.  We need a mutable
                 *    reference to the *front* task only while we are
                 *    processing it.  The mutable borrow is confined to a
                 *    small inner block so that it ends before we move the
                 *    unit itself.
                 * ------------------------------------------------------- */
                if let Some(task) = unit.get_mut_tasks().front_mut() {
                    // `Tasks` currently only has the Digging variant.
                    // If new variants appear later, you can handle them here.
                    let maybe_target: Option<(usize, usize)> = {
                        // -------------------------------------------------
                        // Mutable borrow of the task lives only inside this
                        // block.  When the block ends, `dig` is dropped.
                        // -------------------------------------------------
                        let dig: &mut DiggingTask = match task {
                            Tasks::Digging(d) => d,
                        };

                        dig.progress();

                        // Remember the coordinates of the next tunnel cell.
                        let target: Option<(usize, usize)> = dig.current_target();

                        // If we actually moved, the current step of the
                        // digging task is finished – pop it from the queue.
                        if target.is_some() {
                            dig.pop_front();
                        }

                        target
                    }; // ← `dig` (and the mutable borrow) ends here

                    if let Some((t_row, t_col)) = maybe_target {
                        /* -------------------------------------------------
                         * 3️⃣  We have a target – create the tunnel, move the
                         *    unit, and clean up the source cell.
                         * ------------------------------------------------- */

                        // a) create the tunnel in the target cell
                        self.get_mut_cell(t_row, t_col)
                            .set_structure(Structures::Tunnel(Tunnel::new()));

                        // b) (optional) keep the old coordinates – they are
                        //    no longer needed but kept for possible future use
                        let _old_coords: (usize, usize) = unit.get_coords();

                        // c) move the unit into the target cell
                        *self.get_mut_cell(t_row, t_col).get_mut_option_unit() = Some(unit);

                        // d) the source cell is already empty because of the
                        //    earlier `take()`.  Nothing else to do.
                    } else {
                        // No reachable target – put the unit back where it was.
                        self.get_mut_cell(row, col)
                            .get_mut_option_unit()
                            .replace(unit);
                    }
                } else {
                    // The unit has no tasks – simply put it back.
                    self.get_mut_cell(row, col)
                        .get_mut_option_unit()
                        .replace(unit);
                }
            }
        }
    }
}
