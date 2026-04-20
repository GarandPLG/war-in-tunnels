use crate::app::states::skirmish_states::BoardState;
use clap::ValueEnum;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub board: BoardState,
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
