use crate::app::{
    states::skirmish_states::{
        Players,
        structures::{Mine, Ore, Structure},
    },
    threads::TICK_MS,
};
use ratatui::{style::Color, text::Line};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MineConstruction {
    mine: Mine,
    ticks: u16,
}

impl MineConstruction {
    pub const BUILD_TICKS: u16 = 10000_u16.div_ceil(TICK_MS as u16);

    pub fn new(owner: Players, ore: Ore) -> Option<Self> {
        Mine::new(owner, ore).map(|mine: Mine| -> Self { Self { mine, ticks: 0 } })
    }

    pub fn advance(&mut self) -> () {
        self.ticks = self.ticks.saturating_add(1).min(Self::BUILD_TICKS);
    }

    pub fn is_finished(&self) -> bool {
        self.ticks == Self::BUILD_TICKS
    }

    pub fn progress_percent(&self) -> u16 {
        (u32::from(self.ticks) * 100 / u32::from(Self::BUILD_TICKS)) as u16
    }

    pub fn into_mine(self) -> Option<Mine> {
        self.is_finished().then_some(self.mine)
    }

    pub fn cancel(self) -> Ore {
        self.mine.into_ore()
    }
}

impl Structure for MineConstruction {
    fn get_tag(&self) -> char {
        'C'
    }
    fn get_color(&self) -> Color {
        self.mine.get_color()
    }
    fn get_level(&self) -> char {
        self.mine.get_level()
    }
    fn get_stress(&self) -> u8 {
        self.mine.get_stress()
    }
    fn get_durability(&self) -> u16 {
        self.mine.get_durability()
    }
    fn get_max_durability(&self) -> u16 {
        self.mine.get_max_durability()
    }
    fn get_name(&self) -> &'static str {
        "Mine construction"
    }
    fn get_owner(&self) -> Players {
        self.mine.get_owner()
    }
    fn text(&self) -> Vec<Line<'_>> {
        vec![
            Line::from(format!("Construction: {}%", self.progress_percent())),
            Line::from(format!("Ore: {}", self.mine.get_ore_amount())),
        ]
    }
}
