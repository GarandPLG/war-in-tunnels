use crate::{
    app::states::{
        MainMenuState, PerkDecksState, SettingsState, SkillsConfigState, SkirmishState,
        skirmish_states::{BoardState, Resources},
    },
    cli::Cli,
};
use ratatui::layout::Rect;

#[derive(Debug, Clone, PartialEq)]
pub struct GameStates {
    pub main_menu: MainMenuState,
    pub skirmish: SkirmishState,
    pub perk_decks: PerkDecksState,
    pub skills_config: SkillsConfigState,
    pub settings: SettingsState,
}

impl GameStates {
    pub fn new(args: &Cli, area: &Rect) -> Self {
        Self {
            main_menu: MainMenuState {
                id: 0,
                name: "Main Menu",
                selected_view: 1,
            },
            skirmish: SkirmishState {
                id: 1,
                name: "Skirmish",
                board: BoardState::new(
                    area,
                    args.map_width as usize,
                    args.map_height as usize,
                    args.zoom_level,
                    false,
                ),
                side_panel: false,
                turn_counter: 0,
                player_resources: Resources::new(args.starting_wood, args.starting_metal),
                enemy_resources: Resources::new(args.starting_wood, args.starting_metal),
                building_view: false,
                recruitment_view: false,
                selected_building: 0,
                selected_recruit: 0,
                task_view: false,
                stress_view: false,
                selected_task: 0,
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
                starting_metal: args.starting_metal,
                supply_limit: args.supply_limit.clone(),
                xp_modifier: args.xp_modifier.clone(),
                skill_points_limit: args.skill_points_limit.clone(),
            },
        }
    }
}
