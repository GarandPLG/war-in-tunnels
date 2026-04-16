use crate::app::states::Players;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinerUnit {
    owner: Players,
    tag: &'static str,
}

impl MinerUnit {
    pub fn new(owner: Players) -> Self {
        Self { owner, tag: "M" }
    }

    pub fn get_tag(self) -> &'static str {
        self.tag
    }
}
