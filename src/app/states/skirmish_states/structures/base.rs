use ratatui::style::Color;

use crate::app::states::Players;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseBuilding {
    owner: Players,
    level: u8,
}

impl BaseBuilding {
    pub fn new(owner: Players) -> Self {
        Self { owner, level: b'1' }
    }

    pub fn get_tag(&self) -> char {
        'B'
    }

    pub fn get_color(&self) -> Color {
        match self.owner {
            Players::Player => Color::LightBlue,
            Players::Enemy => Color::LightRed,
        }
    }

    pub fn get_owner(&self) -> Players {
        self.owner
    }

    pub fn get_level(&self) -> char {
        self.level as char
    }
}
