use crate::app::states::skirmish_states::Players;
use ratatui::{
    style::Stylize,
    text::{Line, Span},
};

pub trait Unit {
    fn get_owner(&self) -> Players;

    fn get_tag(&self) -> char;

    fn get_name(&self) -> &'static str;

    fn is_unit(&self) -> bool {
        true
    }

    fn base_text(&self) -> Vec<Line<'_>> {
        let owner: Span<'_> = self.get_owner().get_span();

        vec![Line::from_iter(vec!["Owner: ".gray(), owner])]
    }

    fn text(&self) -> Vec<Line<'_>> {
        self.base_text()
    }
}
