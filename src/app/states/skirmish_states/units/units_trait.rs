use crate::app::states::skirmish_states::Players;

pub trait Unit {
    fn get_owner(&self) -> Players;
    fn get_tag(&self) -> char;
    fn get_name(&self) -> &'static str;
}
