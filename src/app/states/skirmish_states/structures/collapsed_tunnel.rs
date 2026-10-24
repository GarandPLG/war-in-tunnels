use crate::app::states::skirmish_states::{Players, structures::Structure};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CollapsedTunnel {
    durability: u16,
}

impl Default for CollapsedTunnel {
    fn default() -> Self {
        Self::new()
    }
}

impl CollapsedTunnel {
    pub fn new() -> Self {
        Self { durability: 750 }
    }
    pub fn take_damage(&mut self, damage: u16) -> () {
        self.durability = self.durability.saturating_sub(damage);
    }
}

impl Structure for CollapsedTunnel {
    fn get_tag(&self) -> char {
        '#'
    }
    fn get_color(&self) -> Color {
        Color::DarkGray
    }
    fn get_level(&self) -> char {
        ' '
    }
    fn get_stress(&self) -> u8 {
        100
    }
    fn get_durability(&self) -> u16 {
        self.durability
    }
    fn get_max_durability(&self) -> u16 {
        750
    }
    fn get_name(&self) -> &'static str {
        "Collapsed tunnel"
    }
    fn get_owner(&self) -> Players {
        Players::Unclaimed
    }
}
