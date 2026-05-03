use crate::{app::states::skirmish_states::BoardState, cli::Cli};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub board: BoardState,
    pub side_panel: bool,
    pub turn_counter: u64,
}

impl SkirmishState {
    pub fn update(&mut self, cli: &Cli) {
        self.board.advance_turn();

        // if self.board.is_victory() {}

        self.turn_counter += 1;
    }
}
