use crate::app::states::skirmish_states::structures::{BaseBuilding, Stone, Structure, Tunnel};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structures {
    Base(BaseBuilding),
    Tunnel(Tunnel),
    Stone(Stone),
}

impl Structures {
    pub fn get_color(&self) -> Color {
        match self {
            Structures::Base(b) => b.get_color(),
            Structures::Tunnel(t) => t.get_color(),
            Structures::Stone(s) => s.get_color(),
        }
    }

    pub fn get_tag(&self) -> char {
        match self {
            Structures::Base(b) => b.get_tag(),
            Structures::Tunnel(t) => t.get_tag(),
            Structures::Stone(s) => s.get_tag(),
        }
    }

    pub fn get_level(&self) -> char {
        match self {
            Structures::Base(b) => b.get_level(),
            Structures::Tunnel(t) => t.get_level(),
            Structures::Stone(s) => s.get_level(),
        }
    }
}
