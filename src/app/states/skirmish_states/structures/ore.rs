use crate::app::states::skirmish_states::{Players, structures::Structure};
use ratatui::{
  style::{Color, Stylize},
  text::Line,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ore {
  owner: Players,
  ore_amount: u16,
  durability: u16,
  stress: u8,
  level: u8,
}

impl Ore {
  pub fn new(owner: Players, level: u8) -> Self {
    Self {
      owner,
      ore_amount: 5000,
      durability: 1000,
      stress: 0,
      level,
    }
  }

  pub fn get_ore_amount(&self) -> u16 {
    self.ore_amount
  }

  pub fn get_max_ore_amount(&self) -> u16 {
    5000
  }

  pub fn extract(&mut self, amount: u16) -> u16 {
    let extracted: u16 = amount.min(self.ore_amount);
    self.ore_amount -= extracted;
    extracted
  }

  pub fn set_owner(&mut self, new_owner: Players) -> &mut Self {
    self.owner = new_owner;
    self
  }
}

impl Structure for Ore {
  fn get_tag(&self) -> char {
    'O'
  }

  fn get_color(&self) -> Color {
    match self.owner {
      Players::Player => self.owner.get_color(),
      Players::Enemy => self.owner.get_color(),
      Players::Unclaimed => Color::Indexed(217),
    }
  }

  fn get_durability(&self) -> u16 {
    self.durability
  }

  fn get_level(&self) -> char {
    if self.level != 0 {
      (self.level + b'0') as char
    } else {
      ' '
    }
  }

  fn get_max_durability(&self) -> u16 {
    1000
  }

  fn get_name(&self) -> &'static str {
    "Ore"
  }

  fn get_owner(&self) -> Players {
    self.owner
  }

  fn get_stress(&self) -> u8 {
    self.stress
  }

  fn text(&self) -> Vec<Line<'_>> {
    let mut lines: Vec<Line<'_>> = Structure::base_text(self);

    lines.push(Line::from_iter([
      "Ore: ".gray(),
      self.get_ore_amount().to_string().cyan(),
      "/".gray(),
      self.get_max_ore_amount().to_string().cyan(),
    ]));

    lines
  }
}
