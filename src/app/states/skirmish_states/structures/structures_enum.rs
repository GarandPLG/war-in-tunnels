use crate::app::states::skirmish_states::structures::{BaseBuilding, Stone, Structure, Tunnel};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structures {
    Base(BaseBuilding),
    Tunnel(Tunnel),
    Stone(Stone),
}

impl Structures {
    #[inline]
    fn structure(&self) -> &dyn Structure {
        match self {
            Self::Base(b) => b,
            Self::Tunnel(t) => t,
            Self::Stone(s) => s,
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

    fn get_stress(&self) -> u8 {
        self.structure().get_stress()
    }
}
