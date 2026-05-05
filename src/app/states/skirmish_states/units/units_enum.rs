use crate::app::states::skirmish_states::{
    Players,
    units::{MinerUnit, Unit},
};

#[derive(Debug, Clone, Copy, PartialEq)]
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

    fn get_hp(&self) -> u16 {
        self.unit().get_hp()
    }

    fn get_max_hp(&self) -> u16 {
        self.unit().get_max_hp()
    }

    fn get_can_dig(&self) -> bool {
        self.unit().get_can_dig()
    }

    fn get_digging_power(&self) -> f32 {
        self.unit().get_digging_power()
    }
}

impl Unit for Option<Units> {
    fn is_unit(&self) -> bool {
        self.map_or(false, |u| u.is_unit())
    }

    fn get_tag(&self) -> char {
        self.map_or(' ', |u| u.get_tag())
    }

    fn get_name(&self) -> &'static str {
        self.map_or("", |u| u.get_name())
    }

    fn get_owner(&self) -> Players {
        self.map_or(Players::Unclaimed, |u| u.get_owner())
    }

    fn get_hp(&self) -> u16 {
        self.map_or(0, |u| u.get_hp())
    }

    fn get_max_hp(&self) -> u16 {
        self.map_or(0, |u| u.get_max_hp())
    }

    fn get_can_dig(&self) -> bool {
        self.map_or(false, |u| u.get_can_dig())
    }

    fn get_digging_power(&self) -> f32 {
        self.map_or(0.0, |u| u.get_digging_power())
    }
}
