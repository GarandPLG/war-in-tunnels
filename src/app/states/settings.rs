use crate::app::states::{PerkDecks, skirmish_states::GameMode};

#[derive(Debug, Clone, PartialEq)]
pub struct SettingsState {
    pub username: String,
    pub game_mode: GameMode,
    pub perk_deck: PerkDecks,
    pub starting_wood: u16,
    pub starting_metal: u16,
    pub supply_limit: u8,
    pub xp_modifier: f32,
    pub skill_points_limit: u16,
}
