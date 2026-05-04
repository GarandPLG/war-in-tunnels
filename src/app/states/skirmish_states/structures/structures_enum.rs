use crate::app::states::skirmish_states::{
    Players,
    structures::{BaseBuilding, Ore, Stone, Structure, Tunnel},
};
use ratatui::{style::Color, text::Line};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structures {
    Base(BaseBuilding),
    Tunnel(Tunnel),
    Stone(Stone),
    Ore(Ore),
}

impl Structures {
    #[inline]
    fn structure(&self) -> &dyn Structure {
        match self {
            Self::Base(b) => b,
            Self::Tunnel(t) => t,
            Self::Stone(s) => s,
            Self::Ore(o) => o,
        }
    }
}

impl Structure for Structures {
    fn get_color(&self) -> Color {
        self.structure().get_color()
    }

    fn get_tag(&self) -> char {
        self.structure().get_tag()
    }

    fn get_level(&self) -> char {
        self.structure().get_level()
    }

    fn get_name(&self) -> &'static str {
        self.structure().get_name()
    }

    fn get_durability(&self) -> u16 {
        self.structure().get_durability()
    }

    fn get_max_durability(&self) -> u16 {
        self.structure().get_max_durability()
    }

    fn get_stress(&self) -> u8 {
        self.structure().get_stress()
    }

    fn get_owner(&self) -> Players {
        self.structure().get_owner()
    }

    fn text(&self) -> Vec<Line<'_>> {
        self.structure().text()
    }
}
