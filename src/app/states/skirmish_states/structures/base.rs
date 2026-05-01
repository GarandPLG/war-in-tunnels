use crate::app::states::skirmish_states::{Players, structures::Structure};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseBuilding {
    durability: u16,
    max_durability: u16,
    stress: u8,
    owner: Players,
    level: u8,
}

impl BaseBuilding {
    pub fn new(owner: Players) -> Self {
        Self {
            durability: 1500,
            max_durability: 1500,
            stress: 0,
            owner,
            level: b'1',
        }
    }
}

impl Structure for BaseBuilding {
    fn get_tag(&self) -> char {
        'B'
    }

    fn get_color(&self) -> Color {
        match self.owner {
            Players::Player => Color::LightBlue,
            Players::Enemy => Color::LightRed,
            Players::Unclaimed => Color::LightGreen,
        }
    }

    fn get_level(&self) -> char {
        self.level as char
    }

    fn get_durability(&self) -> u16 {
        self.durability
    }

    fn get_max_durability(&self) -> u16 {
        self.max_durability
    }

    fn get_stress(&self) -> u8 {
        self.stress
    }

    fn get_name(&self) -> &'static str {
        "Base"
    }

    fn get_owner(&self) -> Players {
        self.owner
    }
}
