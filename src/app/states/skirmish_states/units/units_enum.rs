use crate::app::states::skirmish_states::units::{MinerUnit, OptionalUnit, Unit};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Miner(MinerUnit),
}

impl Units {
    pub fn get_tag(&self) -> char {
        match self {
            Units::Miner(m) => m.get_tag(),
        }
    }
}

impl OptionalUnit for Option<Units> {
    fn try_get_tag(&self) -> char {
        self.map_or(' ', |u| u.get_tag())
    }
}
