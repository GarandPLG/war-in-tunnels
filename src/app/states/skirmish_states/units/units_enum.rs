use crate::app::states::{
    Players,
    skirmish_states::units::{MinerUnit, Unit},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Units {
    Miner(MinerUnit),
}

impl Units {
    fn unit(&self) -> &dyn Unit {
        match self {
            Self::Miner(m) => m,
        }
    }
}

impl Unit for Units {
    fn get_tag(&self) -> char {
        self.unit().get_tag()
    }

    fn get_name(&self) -> &'static str {
        self.unit().get_name()
    }

    fn get_owner(&self) -> Players {
        self.unit().get_owner()
    }
}

impl Unit for Option<Units> {
    fn get_tag(&self) -> char {
        self.map_or(' ', |u| u.get_tag())
    }

    fn get_name(&self) -> &'static str {
        self.map_or("", |u| u.get_name())
    }

    fn get_owner(&self) -> Players {
        self.map_or(Players::Enemy, |u| u.get_owner())
    }
}
