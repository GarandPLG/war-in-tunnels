use crate::{
    app::states::{
        FocusedCell, MainMenuState, Offset, PerkDecksState, SettingsState, SkillsConfigState,
        SkirmishState,
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
    pub fn new(args: &Cli) -> Self {
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
                map_width: args.map_width as usize,
                map_height: args.map_height as usize,
                board_cells: Vec::new(),
                zoom_level: args.zoom_level,
                focused_cell: FocusedCell::new(0, 0),
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
                username: args.username.clone(),
                game_mode: args.game_mode.clone(),
                perk_deck: args.perk_deck.clone(),
                starting_wood: args.starting_wood.clone(),
                starting_iron: args.starting_iron.clone(),
                supply_limit: args.supply_limit.clone(),
                xp_modifier: args.xp_modifier.clone(),
                skill_points_limit: args.skill_points_limit.clone(),
            },
        }
    }
}
