use crate::{
    app::states::{
        MainMenuState, PerkDecksState, SettingsOption, SettingsState, SettingsValue,
        SkillsConfigState, SkirmishState,
    },
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
