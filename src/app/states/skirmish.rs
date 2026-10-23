use crate::app::states::skirmish_states::{
    BoardState, Players, Resources,
    resources::{MINE_PRODUCTION_TICKS, WOOD_DELIVERY_AMOUNT, WOOD_DELIVERY_TICKS},
    structures::{BuildingTerrain, Mine, Structure, Structures},
    units::MinerUnit,
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
}

impl SkirmishState {
    pub const BUILDING_OPTIONS: &'static [&'static str] = &["Mine"];
    pub const RECRUITMENT_OPTIONS: &'static [&'static str] = &["Miner"];

    pub fn cycle_popup_option(&mut self, next: bool) -> () {
        let (selected, count): (&mut usize, usize) = if self.recruitment_view {
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
        self.board.advance_turn();
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
