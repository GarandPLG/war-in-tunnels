use crate::app::states::skirmish_states::{Players, structures::Structure};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tunnel {
    durability: u16,
    max_durability: u16,
    stress: u8,
    roof_support: bool,
    rail: bool,
    lamp: bool,
}

impl Tunnel {
    pub fn new() -> Self {
        Self {
            durability: 500,
            max_durability: 500,
            stress: 25,
            roof_support: false,
            rail: false,
            lamp: false,
        }
    }
}

impl Structure for Tunnel {
    fn get_tag(&self) -> char {
        'T'
    }

    fn get_color(&self) -> Color {
        Color::Gray
    }

    fn get_level(&self) -> char {
        ' '
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
        "Tunnel"
    }

    fn get_owner(&self) -> Players {
        Players::Unclaimed
    }
}
