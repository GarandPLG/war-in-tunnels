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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum ZoomLevel {
    ZoomedIn,
    Default,
    ZoomedOut,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellTag {
    Base(Players),
    Tunel,
    Stone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Players {
    Player,
    Enemy,
}
