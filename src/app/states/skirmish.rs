use crate::app::states::skirmish_states::BoardState;
use clap::ValueEnum;
use ratatui::{
    style::{Color, Stylize},
    text::Span,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub board: BoardState,
    pub side_panel: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Players {
    Player,
    Enemy,
}

impl Players {
    pub fn get_text(&self) -> &'static str {
        match self {
            Self::Player => "Player",
            Self::Enemy => "Enemy",
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            Self::Player => Color::LightBlue,
            Self::Enemy => Color::LightRed,
        }
    }

    pub fn get_span(&self) -> Span<'static> {
        match self {
            Self::Player => self.get_text().fg(self.get_color()),
            Self::Enemy => self.get_text().fg(self.get_color()),
        }
    }
}
