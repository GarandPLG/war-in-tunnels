#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GameStates {
    pub main_menu_state: MainMenuState,
    pub skirmish_state: SkirmishState,
    pub perk_decks_state: PerkDecksState,
    pub skills_config_state: SkillsConfigState,
    pub settings_state: SettingsState,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SettingsState {
    pub id: usize,
    pub name: &'static str,
    pub selected_setting: usize,
    pub show_popup: bool,
}

impl GameStates {
    pub fn new() -> Self {
        Self {
            main_menu_state: MainMenuState {
                id: 0,
                name: "Main Menu",
                selected_view: 1,
            },
            skirmish_state: SkirmishState {
                id: 1,
                name: "Skirmish",
            },
            perk_decks_state: PerkDecksState {
                id: 2,
                name: "Perk Decks",
                selected_perk_deck: 0,
            },
            skills_config_state: SkillsConfigState {
                id: 3,
                name: "Skills Config",
                selected_skill: 0,
            },
            settings_state: SettingsState {
                id: 4,
                name: "Settings",
                selected_setting: 0,
                show_popup: false,
            },
        }
    }
}
