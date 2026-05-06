use crate::app::states::skirmish_states::{
    Players,
    structures::{BaseBuilding, Ore, Stone, Structure, Tunnel},
};
use ratatui::{
    style::{Color, Stylize},
    text::Line,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Structures {
    Base(BaseBuilding),
    Tunnel(Tunnel),
    Stone(Stone),
    Ore(Ore),
}

impl Structures {
    #[inline]
    fn s(&self) -> &dyn Structure {
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
        self.s().get_color()
    }

    fn get_tag(&self) -> char {
        self.s().get_tag()
    }

    fn get_level(&self) -> char {
        self.s().get_level()
    }

    fn get_name(&self) -> &'static str {
        self.s().get_name()
    }

    fn get_durability(&self) -> u16 {
        self.s().get_durability()
    }

    fn get_max_durability(&self) -> u16 {
        self.s().get_max_durability()
    }

    fn get_stress(&self) -> u8 {
        self.s().get_stress()
    }

    fn get_owner(&self) -> Players {
        self.s().get_owner()
    }

    fn text(&self) -> Vec<Line<'_>> {
        match self.s().get_owner() {
            Players::Unclaimed => self.s().text(),
            Players::Player | Players::Enemy => {
                let mut lines: Vec<Line<'_>> = self.s().text();

                lines.push(Line::from_iter([
                    "Owner: ".gray(),
                    self.s().get_owner().get_span(),
                ]));

                lines
            }
        }
    }
}
