use crate::app::states::skirmish_states::{Players, structures::Structure};
use crate::app::threads::TICK_MS;
use ratatui::{style::Color, text::Line};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseBuilding {
    durability: u16,
    stress: u8,
    owner: Players,
    level: u8,
    queued_miners: u32,
    recruitment_ticks: u16,
    waiting_for_departure: bool,
}

impl BaseBuilding {
    pub const RECRUITMENT_TICKS: u16 = 10000_u16.div_ceil(TICK_MS as u16);

    pub fn queued_miners(&self) -> u32 {
        self.queued_miners
    }

    pub fn recruitment_progress(&self) -> u16 {
        (u32::from(self.recruitment_ticks) * 100 / u32::from(Self::RECRUITMENT_TICKS)) as u16
    }

    pub fn can_queue_miner(&self) -> bool {
        self.owner != Players::Unclaimed && self.queued_miners < u32::MAX
    }

    pub fn queue_miner(&mut self) -> bool {
        if !self.can_queue_miner() {
            return false;
        }
        self.queued_miners += 1;
        true
    }

    pub fn advance_recruitment(&mut self, occupied: bool) -> bool {
        if self.waiting_for_departure {
            if occupied {
                return false;
            }
            self.waiting_for_departure = false;
        }
        if self.queued_miners == 0 {
            return false;
        }
        self.recruitment_ticks = self
            .recruitment_ticks
            .saturating_add(1)
            .min(Self::RECRUITMENT_TICKS);
        if self.recruitment_ticks < Self::RECRUITMENT_TICKS || occupied {
            return false;
        }
        self.queued_miners -= 1;
        self.recruitment_ticks = 0;
        self.waiting_for_departure = true;
        true
    }

    pub fn recruitment_status(&self, occupied: bool) -> &'static str {
        if self.queued_miners == 0 {
            "Queue empty"
        } else if self.waiting_for_departure && occupied {
            "Paused: waiting for Miner to leave base"
        } else if self.recruitment_ticks == Self::RECRUITMENT_TICKS && occupied {
            "Ready: waiting for free base cell"
        } else {
            "Recruiting"
        }
    }

    pub fn new(owner: Players) -> Self {
        Self {
            durability: 1500,
            stress: 0,
            owner,
            level: 1,
            queued_miners: 0,
            recruitment_ticks: 0,
            waiting_for_departure: false,
        }
    }
}

impl Structure for BaseBuilding {
    fn get_tag(&self) -> char {
        'B'
    }

    fn get_color(&self) -> Color {
        match self.owner {
            Players::Player => Color::Rgb(0, 0, 255),
            Players::Enemy => Color::Rgb(255, 0, 0),
            Players::Unclaimed => Color::Rgb(0, 255, 0),
        }
    }

    fn get_level(&self) -> char {
        (self.level + b'0') as char
    }

    fn get_durability(&self) -> u16 {
        self.durability
    }

    fn get_max_durability(&self) -> u16 {
        1500
    }

    fn get_stress(&self) -> u8 {
        self.stress
    }

    fn get_name(&self) -> &'static str {
        "Base"
    }

    fn get_owner(&self) -> Players {
        self.owner
    }

    fn text(&self) -> Vec<Line<'_>> {
        let mut lines: Vec<Line<'_>> = self.base_text();
        lines.push(Line::from(format!(
            "Recruitment queue: {}",
            self.queued_miners
        )));
        if self.queued_miners > 0 {
            lines.push(Line::from(format!(
                "Miner progress: {}%",
                self.recruitment_progress()
            )));
        }
        lines
    }
}
