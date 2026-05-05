use crate::app::states::skirmish_states::{Players, tasks::Tasks, units::Unit};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinerUnit {
    owner: Players,
    hp: u16,
    can_dig: bool,
    digging_power: f32,
    // tasks: Vec<Tasks>,
}

impl MinerUnit {
    pub fn new(owner: Players) -> Self {
        Self {
            owner,
            hp: 100,
            can_dig: true,
            digging_power: 0.55,
            // tasks: Vec::new(),
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
}
