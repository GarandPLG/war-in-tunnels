use crate::app::states::Players;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinerUnit {
    owner: Players,
}

impl MinerUnit {
    pub fn new(owner: Players) -> Self {
        Self { owner }
    }

    pub fn get_tag(self) -> char {
        'M'
    }
}
