use crate::app::states::skirmish_states::{Players, tasks::Tasks, units::Unit};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct MinerUnit {
    owner: Players,
    hp: u16,
    can_dig: bool,
    digging_power: f32,
    coords: (usize, usize),
    tasks: VecDeque<Tasks>,
}

impl MinerUnit {
    pub fn new(owner: Players, coords: (usize, usize)) -> Self {
        Self {
            owner,
            hp: 100,
            can_dig: true,
            digging_power: 0.55,
            coords,
            tasks: VecDeque::new(),
        }
    }
}

impl Unit for MinerUnit {
    fn get_owner(&self) -> Players {
        self.owner
    }

    fn get_tag(&self) -> char {
        'M'
    }

    fn get_name(&self) -> &'static str {
        "Miner"
    }

    fn get_hp(&self) -> u16 {
        self.hp
    }

    fn get_max_hp(&self) -> u16 {
        100
    }

    fn get_can_dig(&self) -> bool {
        self.can_dig
    }

    fn get_digging_power(&self) -> f32 {
        self.digging_power
    }

    fn get_coords(&self) -> (usize, usize) {
        self.coords
    }

    fn get_tasks(&self) -> &VecDeque<Tasks> {
        &self.tasks
    }

    fn get_tasks_formatted(&self) -> String {
        format!("{:?}", self.tasks)
    }

    fn set_task(&mut self, task: Tasks) {
        self.tasks.push_back(task);
    }
}
