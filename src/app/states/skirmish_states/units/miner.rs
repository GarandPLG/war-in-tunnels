use crate::app::states::{Players, skirmish_states::units::Unit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinerUnit {
    owner: Players,
}

impl MinerUnit {
    pub fn new(owner: Players) -> Self {
        Self { owner }
    }
}

impl Unit for MinerUnit {
    fn get_owner(&self) -> Players {
        self.owner
    }

    fn get_tag(&self) -> char {
        'M'
    }
}
