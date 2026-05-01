use crate::app::states::skirmish_states::BoardState;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub board: BoardState,
    pub side_panel: bool,
}
