use crate::{
    app::states::{
        MainMenuState, Offset, PerkDecksState, SettingsState, SkillsConfigState, SkirmishState,
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
                vertical_offset: Offset::new(),
                horizontal_offset: Offset::new(),
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
        }
    }
}
