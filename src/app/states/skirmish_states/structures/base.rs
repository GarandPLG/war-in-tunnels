use crate::app::states::{Players, skirmish_states::structures::Structure};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseBuilding {
    durability: u16,
    stress: u8,
    owner: Players,
    level: u8,
}

impl BaseBuilding {
    pub fn new(owner: Players) -> Self {
        Self {
            durability: 1500,
            stress: 0,
            owner,
            level: b'1',
        }
    }

    pub fn get_owner(&self) -> Players {
        self.owner
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
        }
    }

    fn get_level(&self) -> char {
        self.level as char
    }

    fn get_durability(&self) -> u16 {
        self.durability
    }

    fn get_stress(&self) -> u8 {
        self.stress
    }

    fn get_name(&self) -> &'static str {
        "Base"
    }
}
