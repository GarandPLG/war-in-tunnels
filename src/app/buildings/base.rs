use crate::app::states::Players;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseBuilding {
    owner: Players,
}
