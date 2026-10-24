use crate::app::{
    helpers::cells_area_helper,
    states::{
        FocusedCell, Offset,
        skirmish_states::{
            CellSizes, MarkedCells, MoveFocusedCell, Players, ZoomLevel,
            structures::{
                BaseBuilding, BuildingTerrain, MineConstruction, Ore, Stone, Structure, Structures,
                Tunnel,
            },
            tasks::{DiggingTask, MovingTask, Task, Tasks},
            units::{MinerUnit, Unit, Units},
        },
    },
    widgets::CellWidget,
};
use ratatui::layout::Rect;
use std::collections::VecDeque;

pub struct Excavation {
    pub destination: (usize, usize),
    pub origin: (usize, usize),
    pub owner: Players,
    pub auto_supports: bool,
}

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
    pub auto_supports: bool,
    pub movement_only: bool,
}

impl BoardState {
    pub fn advance_recruitment(&mut self) -> () {
        for (row, cells) in self.cells.iter_mut().enumerate() {
            for (col, cell) in cells.iter_mut().enumerate() {
                if let Structures::Base(mut base) = cell.get_structure() {
                    if base.advance_recruitment(cell.get_ref_option_unit().is_some()) {
                        *cell.get_mut_option_unit() =
                            Some(Units::Miner(MinerUnit::new(base.get_owner(), (row, col))));
                    }
                    cell.set_structure(Structures::Base(base));
                }
            }
        }
    }

    fn adjacent_cells(&self, (row, col): (usize, usize)) -> Vec<&CellWidget> {
        let coordinates: [(usize, usize); 4] = [
            (row.wrapping_sub(1), col),
            (row.saturating_add(1), col),
            (row, col.wrapping_sub(1)),
            (row, col.saturating_add(1)),
        ];
        coordinates
            .into_iter()
            .filter_map(|(row, col): (usize, usize)| -> Option<&CellWidget> {
                self.cells
                    .get(row)
                    .and_then(|cells: &Vec<CellWidget>| -> Option<&CellWidget> { cells.get(col) })
            })
            .collect()
    }

    pub fn has_adjacent_builder(&self, coords: (usize, usize), owner: Players) -> bool {
        owner != Players::Unclaimed
            && self
                .adjacent_cells(coords)
                .iter()
                .any(|cell: &&CellWidget| -> bool {
                    let unit: &Option<Units> = cell.get_ref_option_unit();
                    unit.get_owner() == owner && unit.get_can_build()
                })
    }

    pub fn can_build_at(
        &self,
        (row, col): (usize, usize),
        owner: Players,
        terrain: BuildingTerrain,
    ) -> bool {
        if owner == Players::Unclaimed {
            return false;
        }
        let Some(cell) = self
            .cells
            .get(row)
            .and_then(|cells: &Vec<CellWidget>| -> Option<&CellWidget> { cells.get(col) })
        else {
            return false;
        };
        if cell.get_ref_option_unit().is_some() {
            return false;
        }
        if !self.has_adjacent_builder((row, col), owner) {
            return false;
        }
        match (terrain, cell.get_structure()) {
            (BuildingTerrain::Ore, Structures::Ore(ore)) => {
                ore.get_owner() == Players::Unclaimed && ore.get_ore_amount() > 0
            }
            (BuildingTerrain::Stone, Structures::Stone(_)) => self
                .adjacent_cells((row, col))
                .iter()
                .any(|cell: &&CellWidget| -> bool {
                    matches!(cell.get_structure(), Structures::Tunnel(_))
                        || matches!(cell.get_structure(), Structures::Base(_))
                            && cell.get_structure().get_owner() == owner
                }),
            _ => false,
        }
    }

    pub fn start_mine_construction(&mut self, coords: (usize, usize), owner: Players) -> bool {
        if !self.can_build_at(coords, owner, BuildingTerrain::Ore) {
            return false;
        }
        let cell: &mut CellWidget = self.get_mut_cell(coords.0, coords.1);
        if let Structures::Ore(ore) = cell.get_structure()
            && let Some(site) = MineConstruction::new(owner, ore)
        {
            cell.set_structure(Structures::MineConstruction(site));
            return true;
        }
        false
    }

    pub fn cancel_mine_construction(&mut self, (row, col): (usize, usize), owner: Players) -> bool {
        let Some(cell): Option<&mut CellWidget> = self.cells.get_mut(row).and_then(
            |cells: &mut Vec<CellWidget>| -> Option<&mut CellWidget> { cells.get_mut(col) },
        ) else {
            return false;
        };
        if let Structures::MineConstruction(site) = cell.get_structure()
            && site.get_owner() == owner
        {
            cell.set_structure(Structures::Ore(site.cancel()));
            return true;
        }
        false
    }

    pub fn advance_construction(&mut self) -> () {
        let mut opened: bool = false;
        for row in 0..self.cells.len() {
            for col in 0..self.cells[row].len() {
                if let Structures::MineConstruction(mut site) = self.cells[row][col].get_structure()
                    && self.has_adjacent_builder((row, col), site.get_owner())
                {
                    site.advance();
                    let structure: Structures = if let Some(mine) = site.into_mine() {
                        opened = true;
                        Structures::Mine(mine)
                    } else {
                        Structures::MineConstruction(site)
                    };
                    self.cells[row][col].set_structure(structure);
                }
            }
        }
        if opened {
            self.recalculate_stress();
        }
    }

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
                    Structures::Ore(Ore::new(Players::Unclaimed, 1))
                } else if enemy_ore {
                    Structures::Ore(Ore::new(Players::Unclaimed, 1))
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
            auto_supports: true,
            movement_only: false,
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
        if self.marked_cells.marked_cells.back() == Some(&new_cell) {
            return;
        }
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

    pub fn configure_selected_task(&mut self, movement_only: bool) -> () {
        if !self.marked_cells.marking_cells {
            return;
        }
        if movement_only
            && self
                .marked_cells
                .marked_cells
                .iter()
                .skip(1)
                .any(|&coords: &(usize, usize)| -> bool { !self.is_walkable(coords) })
        {
            let origin: (usize, usize) = self.marked_cells.selected_unit.get_coords();
            let focused: (usize, usize) = self.focused_cell.get_coords();
            self.end_marking_cells(true);
            self.get_mut_cell(focused.0, focused.1).set_selected(false);
            self.focused_cell.set_focused_cell(origin);
            self.marked_cells.marking_cells = true;
            self.start_marking_cells();
        }
        self.movement_only = movement_only;
    }

    pub fn start_marking_cells(&mut self) {
        let (row, col) = self.focused_cell.get_coords();

        let selected_unit: Option<Units> = self.get_ref_cell(row, col).get_option_unit();
        if selected_unit.get_owner() != Players::Player || !selected_unit.get_can_dig() {
            self.marked_cells.marking_cells = false;
            return;
        }
        // Extend queued work from its final destination, not the miner's current cell.
        let start: (usize, usize) = selected_unit
            .get_ref_tasks()
            .back()
            .and_then(|task| task.get_path().back())
            .copied()
            .unwrap_or((row, col));
        self.focused_cell.set_focused_cell(start);
        let start: (usize, usize) = self.focused_cell.get_coords();
        self.get_mut_cell(row, col).set_selected(false);
        self.get_mut_cell(start.0, start.1)
            .set_selected(true)
            .set_marked(true);
        if !self.is_focused_cell_visible() {
            self.vertical_offset
                .set_value(start.0.saturating_sub(self.rows / 2));
            self.horizontal_offset
                .set_value(start.1.saturating_sub(self.cols / 2));
        }
        self.marked_cells.selected_unit = selected_unit;
        self.marked_cells.marked_cells.push_back(start);
    }

    pub fn end_marking_cells(&mut self, del: bool) {
        self.marked_cells.marking_cells = false;
        for (row, col) in self.marked_cells.marked_cells.clone() {
            self.get_mut_cell(row, col).set_marked(false);
        }

        let (row, col) = self.marked_cells.selected_unit.get_coords();

        if self.marked_cells.marked_cells.len() > 1 && !del {
            let path: Vec<(usize, usize)> = self
                .marked_cells
                .marked_cells
                .iter()
                .skip(1)
                .cloned()
                .collect();

            let tick_increase: f32 = self.marked_cells.selected_unit.get_digging_power();

            let tasks: VecDeque<Tasks> = path
                .chunk_by(|a, b| self.is_walkable(*a) == self.is_walkable(*b))
                .map(|segment| {
                    let path: VecDeque<(usize, usize)> = segment.iter().copied().collect();
                    let mut task: Tasks = if self.movement_only || self.is_walkable(segment[0]) {
                        Tasks::Moving(MovingTask::new(path))
                    } else {
                        Tasks::Digging(DiggingTask::new(path, tick_increase))
                    };
                    task.set_auto_supports(self.auto_supports);
                    if let Tasks::Moving(moving) = &mut task {
                        moving.dig_if_blocked = !self.movement_only;
                    }
                    task
                })
                .collect();
            if let Some(unit) = self.get_mut_cell(row, col).get_mut_option_unit().as_mut() {
                unit.get_mut_tasks().extend(tasks);
            }
        }

        self.marked_cells.selected_unit = None;
        self.marked_cells.marked_cells.clear();
        self.movement_only = false;
    }

    fn max_offset(map_size: usize, size: usize) -> usize {
        if map_size > size { map_size - size } else { 0 }
    }

    fn is_walkable(&self, (row, col): (usize, usize)) -> bool {
        self.cells
            .get(row)
            .and_then(|cells| cells.get(col))
            .is_some_and(|cell| {
                self.marked_cells
                    .selected_unit
                    .as_ref()
                    .is_some_and(|unit: &Units| -> bool { cell.get_structure().can_enter(unit) })
            })
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
        if self.marked_cells.marking_cells && self.movement_only && !self.is_walkable(new_cell) {
            self.focused_cell.set_focused_cell(old_cell);
            return;
        }

        self.get_mut_cell(old_cell.0, old_cell.1)
            .set_selected(false);
        self.get_mut_cell(new_cell.0, new_cell.1).set_selected(true);

        if self.marked_cells.marking_cells {
            self.push_marked_cell(new_cell);
        }
    }

    pub fn cancel_focused_tasks(&mut self, all: bool) -> bool {
        if self.marked_cells.marking_cells {
            return false;
        }
        let (row, col): (usize, usize) = self.focused_cell.get_coords();
        let Some(unit) = self.cells[row][col].get_mut_option_unit().as_mut() else {
            return false;
        };
        if unit.get_owner() != Players::Player || unit.get_ref_tasks().is_empty() {
            return false;
        }
        if all {
            unit.get_mut_tasks().clear();
        } else {
            unit.get_mut_tasks().pop_back();
        }
        true
    }

    pub fn advance_turn(&mut self) -> () {
        let _: Vec<Excavation> = self.advance_turn_with_events();
    }

    pub fn advance_turn_with_events(&mut self) -> Vec<Excavation> {
        let mut excavated: Vec<Excavation> = Vec::new();
        // Snapshot positions so moving to a later cell cannot grant another turn.
        let active: Vec<(usize, usize)> = self
            .cells
            .iter()
            .enumerate()
            .flat_map(|(row, cells)| {
                cells.iter().enumerate().filter_map(move |(col, cell)| {
                    cell.get_ref_option_unit()
                        .as_ref()
                        .filter(|unit| !unit.get_ref_tasks().is_empty())
                        .map(|_| (row, col))
                })
            })
            .collect();

        for (row, col) in active {
            if self.working_on_support((row, col)) {
                continue;
            }
            // Keep the route's origin stable while the player is drawing it.
            if self.marked_cells.marking_cells
                && self.marked_cells.selected_unit.get_coords() == (row, col)
            {
                continue;
            }
            let Some(mut unit) = self.get_mut_cell(row, col).get_mut_option_unit().take() else {
                continue;
            };
            let mut task: Tasks = unit.get_mut_tasks().pop_front().unwrap();
            let mut destination = (row, col);

            if let Some((target_row, target_col)) = task.get_path().front().copied() {
                let adjacent: bool = row
                    .abs_diff(target_row)
                    .saturating_add(col.abs_diff(target_col))
                    == 1;
                if target_row >= self.map_height || target_col >= self.map_width || !adjacent {
                    // Discard invalid routes rather than teleporting or indexing outside the map.
                    *self.get_mut_cell(row, col).get_mut_option_unit() = Some(unit);
                    continue;
                }

                let cell: &mut CellWidget = self.get_mut_cell(target_row, target_col);
                if cell.get_ref_option_unit().is_none() {
                    let structure: Structures = cell.get_structure();
                    let walkable: bool = structure.can_enter(&unit);
                    // Previously planned stone may already be a tunnel when this task starts.
                    task = match (task, structure) {
                        (Tasks::Digging(dig), _) if walkable => {
                            Tasks::Moving(MovingTask::from_digging(dig))
                        }
                        (
                            Tasks::Moving(moving),
                            Structures::Stone(_) | Structures::CollapsedTunnel(_),
                        ) if moving.dig_if_blocked => Tasks::Digging(DiggingTask::from_moving(
                            moving,
                            unit.get_digging_power(),
                        )),
                        (task, _) => task,
                    };
                    match (&mut task, structure) {
                        (
                            Tasks::Digging(dig),
                            Structures::Stone(_) | Structures::CollapsedTunnel(_),
                        ) => {
                            let mut obstacle: Structures = structure;
                            let max: f32 = f32::from(Stone::new().get_max_durability());
                            let before: u16 = (dig.get_progress() * max / 100.0) as u16;
                            dig.progress();
                            let after: u16 = (dig.get_progress() * max / 100.0) as u16;
                            obstacle.take_damage(after.saturating_sub(before));
                            if obstacle.get_durability() == 0 {
                                cell.set_structure(Structures::Tunnel(Tunnel::new()));
                                excavated.push(Excavation {
                                    destination: (target_row, target_col),
                                    origin: (row, col),
                                    owner: unit.get_owner(),
                                    auto_supports: dig.auto_supports(),
                                });
                                destination = (target_row, target_col);
                                dig.pop_front();
                            } else {
                                cell.set_structure(obstacle);
                            }
                        }
                        (Tasks::Moving(moving), _) if walkable => {
                            moving.progress();
                            if moving.is_finished() {
                                destination = (target_row, target_col);
                                moving.pop_front();
                            }
                        }
                        _ => {}
                    }
                }
                if !task.get_path().is_empty() {
                    unit.get_mut_tasks().push_front(task);
                }
            }
            unit.set_coords(destination);
            *self
                .get_mut_cell(destination.0, destination.1)
                .get_mut_option_unit() = Some(unit);
        }
        excavated
    }
}
