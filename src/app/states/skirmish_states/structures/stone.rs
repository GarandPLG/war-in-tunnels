use crate::app::states::skirmish_states::{Players, structures::Structure};
use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stone {
  durability: u16,
  stress: u8,
}

impl Stone {
  pub fn take_damage(&mut self, damage: u16) {
    self.durability = self.durability.saturating_sub(damage);
  }

  pub fn new() -> Self {
    Self {
      durability: 1000,
      stress: 0,
    }
  }
}

impl Structure for Stone {
  fn get_tag(&self) -> char {
    ' '
  }

  fn get_color(&self) -> Color {
    Color::White
  }

  fn get_level(&self) -> char {
    ' '
  }

  fn get_durability(&self) -> u16 {
    self.durability
  }

  fn get_max_durability(&self) -> u16 {
    1000
  }

  fn get_stress(&self) -> u8 {
    self.stress
  }

  fn get_name(&self) -> &'static str {
    "Stone"
  }

  fn get_owner(&self) -> Players {
    Players::Unclaimed
  }
}
