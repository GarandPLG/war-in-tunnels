use crate::app::states::skirmish_states::{Players, structures::Structure};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ore {
    owner: Players,
    ore_amount: u16,
    // ore_max_amount: u16,
    durability: u16,
    stress: u8,
}

impl Ore {
    pub fn new(owner: Players) -> Self {
        Self {
            owner,
            ore_amount: 1000,
            durability: 1000,
            stress: 0,
        }
    }

    pub fn get_ore_amount(&self) -> u16 {
        self.ore_amount
    }

    pub fn get_max_ore_amount(&self) -> u16 {
        1000
    }
}

impl Structure for Ore {
    fn get_tag(&self) -> char {
        'O'
    }

    fn get_color(&self) -> Color {
        match self.owner {
            Players::Player => self.owner.get_color(),
            Players::Enemy => self.owner.get_color(),
            Players::Unclaimed => Color::Indexed(217),
        }
    }

    fn get_durability(&self) -> u16 {
        self.durability
    }

    fn get_level(&self) -> char {
        ' '
    }

    fn get_max_durability(&self) -> u16 {
        1000
    }

    fn get_name(&self) -> &'static str {
        "Ore"
    }

    fn get_owner(&self) -> Players {
        self.owner
    }

    fn get_stress(&self) -> u8 {
        self.stress
    }
}
