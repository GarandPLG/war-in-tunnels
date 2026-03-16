use crate::app::states::PerkDecks;
use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsState {
    pub id: usize,
    pub name: &'static str,
    pub selected_setting: usize,
    pub show_popup: bool,
    pub selected_setting_new_value: String,
    pub error_message: String,
    pub options: Vec<SettingsOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsOption {
    pub name: &'static str,
    pub value: SettingsValue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SettingsValue {
    U8(u8),
    F32(f32),
    U16(u16),
    Text(String),
    GameMode(GameMode),
    PerkDeck(PerkDecks),
}

impl Display for SettingsValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsValue::U8(v) => write!(f, "{}", v),
            SettingsValue::F32(v) => write!(f, "{}", v),
            SettingsValue::U16(v) => write!(f, "{}", v),
            SettingsValue::Text(v) => write!(f, "{}", v),
            SettingsValue::GameMode(v) => write!(f, "{}", v),
            SettingsValue::PerkDeck(v) => write!(f, "{}", v),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::FrontLines => write!(f, "Front Lines"),
            GameMode::LastManStanding => write!(f, "Last Man Standing"),
        }
    }
}
