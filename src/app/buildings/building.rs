use crate::app::buildings::BaseBuilding;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Buildings {
    Base(BaseBuilding),
}
