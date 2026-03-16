use std::fmt::Display;

use crate::{
    app::{GameMode, PerkDecks},
    cli::Cli,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GameStates {
    pub main_menu: MainMenuState,
    pub skirmish: SkirmishState,
    pub perk_decks: PerkDecksState,
    pub skills_config: SkillsConfigState,
    pub settings: SettingsState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MainMenuState {
    pub id: usize,
    pub name: &'static str,
    pub selected_view: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PerkDecksState {
    pub id: usize,
    pub name: &'static str,
    pub selected_perk_deck: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkillsConfigState {
    pub id: usize,
    pub name: &'static str,
    pub selected_skill: usize,
}

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
pub enum SettingsValue {
    U8(u8),
    F32(f32),
    U16(u16),
    Text(String),
    GameMode(GameMode),
    PerkDeck(PerkDecks),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsOption {
    pub name: &'static str,
    pub value: SettingsValue,
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

impl Display for GameMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameMode::FrontLines => write!(f, "Front Lines"),
            GameMode::LastManStanding => write!(f, "Last Man Standing"),
        }
    }
}

impl Display for PerkDecks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PerkDecks::Silesian => write!(f, "Silesian"),
            PerkDecks::BogeyMan => write!(f, "Bogey Man"),
            PerkDecks::Anteater => write!(f, "Anteater"),
        }
    }
}

impl GameStates {
    pub fn new(args: Cli) -> Self {
        Self {
            main_menu: MainMenuState {
                id: 0,
                name: "Main Menu",
                selected_view: 1,
            },
            skirmish: SkirmishState {
                id: 1,
                name: "Skirmish",
            },
            perk_decks: PerkDecksState {
                id: 2,
                name: "Perk Decks",
                selected_perk_deck: 0,
            },
            skills_config: SkillsConfigState {
                id: 3,
                name: "Skills Config",
                selected_skill: 0,
            },
            settings: SettingsState {
                id: 4,
                name: "Settings",
                selected_setting: 0,
                show_popup: false,
                selected_setting_new_value: String::new(),
                error_message: String::new(),
                options: vec![
                    SettingsOption {
                        name: "Username",
                        value: SettingsValue::Text(args.username),
                    },
                    SettingsOption {
                        name: "Game Mode",
                        value: SettingsValue::GameMode(args.game_mode),
                    },
                    SettingsOption {
                        name: "Map Width",
                        value: SettingsValue::U8(args.map_width),
                    },
                    SettingsOption {
                        name: "Map Height",
                        value: SettingsValue::U8(args.map_height),
                    },
                    SettingsOption {
                        name: "Perk Deck",
                        value: SettingsValue::PerkDeck(args.perk_deck),
                    },
                    SettingsOption {
                        name: "Starting Wood",
                        value: SettingsValue::U8(args.starting_wood),
                    },
                    SettingsOption {
                        name: "Starting Iron",
                        value: SettingsValue::U8(args.starting_iron),
                    },
                    SettingsOption {
                        name: "Supply Limit",
                        value: SettingsValue::U8(args.supply_limit),
                    },
                    SettingsOption {
                        name: "XP Modifier",
                        value: SettingsValue::F32(args.xp_modifier),
                    },
                    SettingsOption {
                        name: "Skill Points Limit",
                        value: SettingsValue::U16(args.skill_points_limit),
                    },
                ],
            },
        }
    }
}
