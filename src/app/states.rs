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
pub struct SettingsOptions {
    pub username: String,
    pub game_mode: GameMode,
    pub map_width: u8,
    pub map_height: u8,
    pub perk_deck: PerkDecks,
    pub starting_wood: u8,
    pub starting_iron: u8,
    pub supply_limit: u8,
    pub xp_modifier: f32,
    pub skill_points_limit: u16,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsState {
    pub id: usize,
    pub name: &'static str,
    pub options: SettingsOptions,
    pub selected_setting: usize,
    pub show_popup: bool,
    pub selected_setting_new_value: String,
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
                options: SettingsOptions {
                    username: args.username,
                    game_mode: args.game_mode,
                    map_width: args.map_width,
                    map_height: args.map_height,
                    perk_deck: args.perk_deck,
                    starting_wood: args.starting_wood,
                    starting_iron: args.starting_iron,
                    supply_limit: args.supply_limit,
                    xp_modifier: args.xp_modifier,
                    skill_points_limit: args.skill_points_limit,
                },
            },
        }
    }
}
