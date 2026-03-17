use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}
