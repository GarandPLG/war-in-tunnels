use crate::app::states::skirmish_states::{
  Players, Resources,
  structures::{Ore, Structure},
  units::{Unit, Units},
};
use ratatui::{
  style::{Color, Stylize},
  text::Line,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mine {
  owner: Players,
  ore: Ore,
}

impl Mine {
  pub const COST: Resources = Resources {
    wood: 20,
    metal: 10,
  };
  pub const METAL_PER_CYCLE: u16 = 1;
  pub const STAFFED_METAL_PER_CYCLE: u16 = 2;

  pub fn new(owner: Players, ore: Ore) -> Option<Self> {
    if owner == Players::Unclaimed || ore.get_owner() != Players::Unclaimed {
      return None;
    }
    Some(Self { owner, ore })
  }

  pub fn get_ore_amount(&self) -> u16 {
    self.ore.get_ore_amount()
  }

  pub fn into_ore(self) -> Ore {
    self.ore
  }

  pub fn produce(&mut self) -> u16 {
    self.produce_with_miner(false)
  }

  pub fn accepts_unit(&self, unit: &Units) -> bool {
    matches!(unit, Units::Miner(_)) && unit.get_owner() == self.owner
  }

  pub fn production_per_cycle(&self, staffed: bool) -> u16 {
    let rate: u16 = if staffed {
      Self::STAFFED_METAL_PER_CYCLE
    } else {
      Self::METAL_PER_CYCLE
    };
    rate.min(self.get_ore_amount())
  }

  pub fn produce_with_miner(&mut self, staffed: bool) -> u16 {
    self.ore.extract(self.production_per_cycle(staffed))
  }
}

impl Structure for Mine {
  fn get_tag(&self) -> char {
    'M'
  }
  fn get_color(&self) -> Color {
    self.owner.get_color()
  }
  fn get_level(&self) -> char {
    self.ore.get_level()
  }
  fn get_stress(&self) -> u8 {
    self.ore.get_stress()
  }
  fn get_durability(&self) -> u16 {
    self.ore.get_durability()
  }
  fn get_max_durability(&self) -> u16 {
    self.ore.get_max_durability()
  }
  fn get_name(&self) -> &'static str {
    "Mine"
  }
  fn get_owner(&self) -> Players {
    self.owner
  }
  fn text(&self) -> Vec<Line<'_>> {
    let mut lines: Vec<Line<'_>> = self.base_text();
    lines.push(Line::from_iter([
      "Ore: ".gray(),
      self.get_ore_amount().to_string().cyan(),
      format!("/{}", self.ore.get_max_ore_amount()).gray(),
    ]));
    lines
  }
}
