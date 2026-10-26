use crate::app::states::skirmish_states::{Players, Resources, structures::Structure};
use crate::app::threads::TICK_MS;
use ratatui::{
    style::{Color, Stylize},
    text::Line,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupportWork {
    pub owner: Players,
    pub removing: bool,
    ticks: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tunnel {
    durability: u16,
    stress: u8,
    roof_support: bool,
    support_work: Option<SupportWork>,
    rail: bool,
    lamp: bool,
}

impl Tunnel {
    pub const SUPPORT_WORK_TICKS: u16 = 3000_u16.div_ceil(TICK_MS as u16);
    pub const SUPPORT_COST: Resources = Resources { wood: 10, metal: 2 };

    pub fn has_support(&self) -> bool {
        self.roof_support
    }

    pub fn support_work(&self) -> Option<SupportWork> {
        self.support_work
    }

    pub fn start_support_work(&mut self, owner: Players, removing: bool) -> bool {
        if owner == Players::Unclaimed
            || self.support_work.is_some()
            || self.roof_support != removing
        {
            return false;
        }
        self.support_work = Some(SupportWork {
            owner,
            removing,
            ticks: 0,
        });
        true
    }

    pub fn support_progress(&self) -> Option<u16> {
        self.support_work
            .map(|work: SupportWork| -> u16 { work.ticks * 100 / Self::SUPPORT_WORK_TICKS })
    }

    pub fn advance_support_work(&mut self) -> Option<bool> {
        let work: &mut SupportWork = self.support_work.as_mut()?;
        work.ticks += 1;
        if work.ticks < Self::SUPPORT_WORK_TICKS {
            return None;
        }
        let removing: bool = work.removing;
        self.roof_support = !removing;
        self.support_work = None;
        Some(removing)
    }

    pub fn set_support(&mut self, supported: bool) -> () {
        self.roof_support = supported;
    }

    pub fn set_stress(&mut self, stress: u8) -> () {
        self.stress = stress.min(100);
    }

    pub fn new() -> Self {
        Self {
            durability: 500,
            stress: 20,
            roof_support: false,
            support_work: None,
            rail: false,
            lamp: false,
        }
    }
}

impl Structure for Tunnel {
    fn text(&self) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = self.base_text();
        if let Some(work) = self.support_work {
            lines.push(
                Line::from(format!(
                    "{} support: {}%",
                    if work.removing {
                        "Removing"
                    } else {
                        "Building"
                    },
                    self.support_progress().unwrap_or(0)
                ))
                .fg(Color::Rgb(0, 128, 128)),
            );
        }
        lines
    }

    fn get_tag(&self) -> char {
        'T'
    }

    fn get_color(&self) -> Color {
        Color::Rgb(192, 192, 192)
    }

    fn get_level(&self) -> char {
        if self.roof_support { 'T' } else { ' ' }
    }

    fn get_durability(&self) -> u16 {
        self.durability
    }

    fn get_max_durability(&self) -> u16 {
        500
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
