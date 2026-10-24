use crate::app::states::skirmish_states::{
    BoardState, Excavation, Players, Resources,
    resources::{MINE_PRODUCTION_TICKS, WOOD_DELIVERY_AMOUNT, WOOD_DELIVERY_TICKS},
    structures::{BuildingTerrain, Mine, Structure, Structures, Tunnel},
    tasks::{Task, Tasks},
    units::{MinerUnit, Unit},
};
use crate::app::widgets::CellWidget;

#[derive(Debug, Clone, PartialEq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub board: BoardState,
    pub side_panel: bool,
    pub turn_counter: u64,
    pub player_resources: Resources,
    pub enemy_resources: Resources,
    pub building_view: bool,
    pub recruitment_view: bool,
    pub selected_building: usize,
    pub selected_recruit: usize,
    pub task_view: bool,
    pub stress_view: bool,
    pub selected_task: usize,
}

impl SkirmishState {
    pub const BUILDING_OPTIONS: &'static [&'static str] = &["Mine", "Support"];
    pub const RECRUITMENT_OPTIONS: &'static [&'static str] = &["Miner"];
    pub const TASK_OPTIONS: &'static [&'static str] = &["Digging", "Moving"];

    pub fn can_manage_support(&self, coords: (usize, usize), owner: Players) -> bool {
        owner != Players::Unclaimed
            && self
                .board
                .cells
                .get(coords.0)
                .and_then(|row: &Vec<CellWidget>| -> Option<&CellWidget> { row.get(coords.1) })
                .is_some_and(|cell: &CellWidget| -> bool {
                    matches!(cell.get_structure(), Structures::Tunnel(_))
                        && ((cell.get_ref_option_unit().get_owner() == owner
                            && cell.get_ref_option_unit().get_can_build())
                            || self.board.has_adjacent_builder(coords, owner))
                })
    }

    pub fn can_build_support(&self, coords: (usize, usize), owner: Players) -> bool {
        let resources: &Resources = match owner {
            Players::Player => &self.player_resources,
            Players::Enemy => &self.enemy_resources,
            Players::Unclaimed => return false,
        };
        self.can_manage_support(coords, owner)
            && resources.can_afford(Tunnel::SUPPORT_COST)
            && matches!(self.board.cells[coords.0][coords.1].get_structure(), Structures::Tunnel(tunnel) if !tunnel.has_support() && tunnel.support_work().is_none())
    }

    pub fn build_support(&mut self, coords: (usize, usize), owner: Players) -> bool {
        if !self.can_build_support(coords, owner) {
            return false;
        }
        let Structures::Tunnel(mut tunnel) = self.board.cells[coords.0][coords.1].get_structure()
        else {
            return false;
        };
        let resources: &mut Resources = if owner == Players::Player {
            &mut self.player_resources
        } else {
            &mut self.enemy_resources
        };
        if !resources.spend(Tunnel::SUPPORT_COST) {
            return false;
        }
        tunnel.start_support_work(owner, false);
        self.board.cells[coords.0][coords.1].set_structure(Structures::Tunnel(tunnel));
        true
    }

    pub fn remove_support(&mut self, coords: (usize, usize), owner: Players) -> bool {
        if !self.can_manage_support(coords, owner) {
            return false;
        }
        let Structures::Tunnel(mut tunnel) = self.board.cells[coords.0][coords.1].get_structure()
        else {
            return false;
        };
        if !tunnel.has_support() || tunnel.support_work().is_some() {
            return false;
        }
        tunnel.start_support_work(owner, true);
        self.board.cells[coords.0][coords.1].set_structure(Structures::Tunnel(tunnel));
        true
    }

    pub fn advance_support_work(&mut self) -> () {
        let mut workers: std::collections::HashSet<(usize, usize)> =
            std::collections::HashSet::new();
        let mut finished: bool = false;
        let mut removed: bool = false;
        for row in 0..self.board.map_height {
            for col in 0..self.board.map_width {
                let Structures::Tunnel(mut tunnel) = self.board.cells[row][col].get_structure()
                else {
                    continue;
                };
                let Some(work) = tunnel.support_work() else {
                    continue;
                };
                let Some(worker) = self.board.support_worker((row, col), work.owner) else {
                    continue;
                };
                if !workers.insert(worker) {
                    continue;
                }
                if let Some(removing) = tunnel.advance_support_work() {
                    finished = true;
                    removed |= removing;
                }
                self.board.cells[row][col].set_structure(Structures::Tunnel(tunnel));
            }
        }
        if removed {
            self.board.resolve_collapses();
        } else if finished {
            self.board.recalculate_stress();
        }
    }

    fn prepare_automatic_supports(&mut self) -> () {
        let mut candidates: Vec<((usize, usize), Players)> = Vec::new();
        for (row, cells) in self.board.cells.iter().enumerate() {
            for (col, cell) in cells.iter().enumerate() {
                if !matches!(cell.get_structure(), Structures::Tunnel(tunnel) if !tunnel.has_support() && tunnel.get_stress() == 80)
                {
                    continue;
                }
                if self.board.working_on_support((row, col))
                    || (self.board.marked_cells.marking_cells
                        && self.board.marked_cells.selected_unit.get_coords() == (row, col))
                {
                    continue;
                }
                let Some(unit) = cell.get_ref_option_unit() else {
                    continue;
                };
                let Some(task) = unit.get_ref_tasks().front() else {
                    continue;
                };
                let Some(&(r, c)) = task.get_path().front() else {
                    continue;
                };
                if task.auto_supports()
                    && self.can_build_support((row, col), unit.get_owner())
                    && (matches!(task, Tasks::Digging(_))
                        || matches!(task, Tasks::Moving(moving) if moving.dig_if_blocked))
                    && r < self.board.map_height
                    && c < self.board.map_width
                    && row.abs_diff(r) + col.abs_diff(c) == 1
                    && self.board.cells[r][c].get_ref_option_unit().is_none()
                    && matches!(
                        self.board.cells[r][c].get_structure(),
                        Structures::Stone(_) | Structures::CollapsedTunnel(_)
                    )
                    && self.board.prospective_tunnel_stress((r, c)) >= 100
                {
                    candidates.push(((row, col), unit.get_owner()));
                }
            }
        }
        for (coords, owner) in candidates {
            self.build_support(coords, owner);
        }
    }

    pub fn cycle_popup_option(&mut self, next: bool) -> () {
        let (selected, count): (&mut usize, usize) = if self.task_view {
            (&mut self.selected_task, Self::TASK_OPTIONS.len())
        } else if self.recruitment_view {
            (&mut self.selected_recruit, Self::RECRUITMENT_OPTIONS.len())
        } else if self.building_view {
            (&mut self.selected_building, Self::BUILDING_OPTIONS.len())
        } else {
            return;
        };
        *selected = if next {
            (*selected + 1) % count
        } else {
            (*selected + count - 1) % count
        };
    }

    pub fn can_recruit_miner(&self, (row, col): (usize, usize), owner: Players) -> bool {
        let resources: &Resources = match owner {
            Players::Player => &self.player_resources,
            Players::Enemy => &self.enemy_resources,
            Players::Unclaimed => return false,
        };
        if !resources.can_afford(MinerUnit::RECRUIT_COST) {
            return false;
        }
        self.board.cells.get(row).and_then(|cells: &Vec<CellWidget>| -> Option<&CellWidget> { cells.get(col) })
      .is_some_and(|cell: &CellWidget| -> bool {
        matches!(cell.get_structure(), Structures::Base(base) if base.get_owner() == owner && base.can_queue_miner())
      })
    }

    pub fn recruit_miner(&mut self, coords: (usize, usize), owner: Players) -> bool {
        if !self.can_recruit_miner(coords, owner) {
            return false;
        }
        let cell: &mut CellWidget = &mut self.board.cells[coords.0][coords.1];
        let Structures::Base(mut base) = cell.get_structure() else {
            return false;
        };
        if !base.queue_miner() {
            return false;
        }
        cell.set_structure(Structures::Base(base));
        let resources: &mut Resources = if owner == Players::Player {
            &mut self.player_resources
        } else {
            &mut self.enemy_resources
        };
        resources.spend(MinerUnit::RECRUIT_COST)
    }

    pub fn can_start_mine_construction(&self, coords: (usize, usize), owner: Players) -> bool {
        let resources: &Resources = match owner {
            Players::Player => &self.player_resources,
            Players::Enemy => &self.enemy_resources,
            Players::Unclaimed => return false,
        };
        resources.can_afford(Mine::COST)
            && self.board.can_build_at(coords, owner, BuildingTerrain::Ore)
    }

    pub fn start_mine_construction(&mut self, coords: (usize, usize), owner: Players) -> bool {
        if !self.can_start_mine_construction(coords, owner)
            || !self.board.start_mine_construction(coords, owner)
        {
            return false;
        }
        let resources: &mut Resources = match owner {
            Players::Player => &mut self.player_resources,
            Players::Enemy => &mut self.enemy_resources,
            Players::Unclaimed => return false,
        };
        resources.spend(Mine::COST)
    }

    pub fn cancel_mine_construction(&mut self, coords: (usize, usize), owner: Players) -> bool {
        if owner == Players::Unclaimed || !self.board.cancel_mine_construction(coords, owner) {
            return false;
        }
        let resources: &mut Resources = if owner == Players::Player {
            &mut self.player_resources
        } else {
            &mut self.enemy_resources
        };
        resources.add_wood(Mine::COST.wood);
        resources.add_metal(Mine::COST.metal);
        true
    }

    pub fn tick_update(&mut self) -> () {
        self.advance_support_work();
        self.prepare_automatic_supports();
        let excavated: Vec<Excavation> = self.board.advance_turn_with_events();
        if !excavated.is_empty() {
            self.board.resolve_collapses();
        }
        self.board.advance_recruitment();
        self.board.advance_construction();

        // if self.board.is_victory() {}

        self.turn_counter += 1;
        if self.turn_counter.is_multiple_of(WOOD_DELIVERY_TICKS) {
            self.player_resources.add_wood(WOOD_DELIVERY_AMOUNT);
        }
        if self.turn_counter.is_multiple_of(MINE_PRODUCTION_TICKS) {
            for row in &mut self.board.cells {
                for cell in row {
                    if let Structures::Mine(mut mine) = cell.get_structure() {
                        let staffed: bool = cell.get_ref_option_unit().as_ref().is_some_and(
                            |unit: &crate::app::states::skirmish_states::units::Units| -> bool {
                                mine.accepts_unit(unit)
                            },
                        );
                        let amount: u32 = u32::from(mine.produce_with_miner(staffed));
                        match mine.get_owner() {
                            Players::Player => self.player_resources.add_metal(amount),
                            Players::Enemy => self.enemy_resources.add_metal(amount),
                            Players::Unclaimed => (),
                        }
                        cell.set_structure(Structures::Mine(mine));
                    }
                }
            }
        }
    }
}
