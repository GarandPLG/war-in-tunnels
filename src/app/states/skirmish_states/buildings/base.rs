use crate::app::states::Players;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseBuilding {
    owner: Players,
    level: u8,
    tag: &'static str,
}

impl BaseBuilding {
    pub fn new(owner: Players) -> Self {
        Self {
            owner,
            level: 1,
            tag: "B",
        }
    }

    pub fn get_owner(&self) -> Players {
        self.owner
    }

    pub fn get_tag(&self) -> &'static str {
        self.tag
    }

    pub fn get_level(&self) -> &'static str {
        match self.level {
            1 => "1",
            2 => "2",
            3 => "3",
            _ => " ",
        }
    }
}
