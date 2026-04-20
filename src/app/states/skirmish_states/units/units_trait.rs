use crate::app::states::Players;

pub trait Unit {
    fn get_owner(&self) -> Players;
    fn get_tag(&self) -> char;
}

pub trait OptionalUnit {
    fn try_get_tag(&self) -> char;
}
