use clap::ValueEnum;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PerkDecksState {
    pub id: usize,
    pub name: &'static str,
    pub selected_perk_deck: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum PerkDecks {
    Silesian,
    BogeyMan,
    Anteater,
}

impl Display for PerkDecks {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            PerkDecks::Silesian => write!(f, "Silesian"),
            PerkDecks::BogeyMan => write!(f, "Bogey Man"),
            PerkDecks::Anteater => write!(f, "Anteater"),
        }
    }
}
