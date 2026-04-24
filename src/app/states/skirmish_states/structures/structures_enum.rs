use crate::app::states::skirmish_states::structures::{BaseBuilding, Stone, Structure, Tunnel};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structures {
    Base(BaseBuilding),
    Tunnel(Tunnel),
    Stone(Stone),
}

impl Structure for Structures {
    fn get_color(&self) -> Color {
        match self {
            Self::Base(b) => b.get_color(),
            Self::Tunnel(t) => t.get_color(),
            Self::Stone(s) => s.get_color(),
        }
    }

    fn get_tag(&self) -> char {
        match self {
            Self::Base(b) => b.get_tag(),
            Self::Tunnel(t) => t.get_tag(),
            Self::Stone(s) => s.get_tag(),
        }
    }

    fn get_level(&self) -> char {
        match self {
            Self::Base(b) => b.get_level(),
            Self::Tunnel(t) => t.get_level(),
            Self::Stone(s) => s.get_level(),
        }
    }

    fn get_name(&self) -> &'static str {
        match self {
            Self::Base(b) => b.get_name(),
            Self::Tunnel(t) => t.get_name(),
            Self::Stone(s) => s.get_name(),
        }
    }

    fn get_durability(&self) -> u16 {
        match self {
            Self::Base(b) => b.get_durability(),
            Self::Tunnel(t) => t.get_durability(),
            Self::Stone(s) => s.get_durability(),
        }
    }

    fn get_stress(&self) -> u8 {
        match self {
            Self::Base(b) => b.get_stress(),
            Self::Tunnel(t) => t.get_stress(),
            Self::Stone(s) => s.get_stress(),
        }
    }
}
