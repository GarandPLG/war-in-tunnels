use crate::app::states::skirmish_states::{
    Players,
    structures::{
        BaseBuilding, CollapsedTunnel, Mine, MineConstruction, Ore, Stone, Structure, Tunnel,
    },
    units::Units,
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
    CollapsedTunnel(CollapsedTunnel),
    Ore(Ore),
    Mine(Mine),
    MineConstruction(MineConstruction),
}

impl Structures {
    pub fn take_damage(&mut self, damage: u16) -> () {
        match self {
            Self::Stone(stone) => stone.take_damage(damage),
            Self::CollapsedTunnel(rubble) => rubble.take_damage(damage),
            _ => (),
        }
    }
    pub fn can_enter(&self, unit: &Units) -> bool {
        match self {
            Self::Base(_) | Self::Tunnel(_) => true,
            Self::Mine(mine) => mine.accepts_unit(unit),
            _ => false,
        }
    }
    pub fn build_mine(&mut self, owner: Players) -> bool {
        if let Self::Ore(ore) = *self
            && let Some(mine) = Mine::new(owner, ore)
        {
            *self = Self::Mine(mine);
            return true;
        }
        false
    }

    #[inline]
    fn s(&self) -> &dyn Structure {
        match self {
            Self::Base(b) => b,
            Self::Tunnel(t) => t,
            Self::Stone(s) => s,
            Self::CollapsedTunnel(rubble) => rubble,
            Self::Ore(o) => o,
            Self::Mine(mine) => mine,
            Self::MineConstruction(site) => site,
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
                    "Owner: ".fg(Color::Rgb(192, 192, 192)),
                    self.s().get_owner().get_span(),
                ]));

                lines
            }
        }
    }
}
