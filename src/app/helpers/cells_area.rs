use crate::app::views::skirmish_layout;
use ratatui::{
    layout::{Margin, Rect},
    widgets::{Block, Borders},
};

pub fn cells_area_helper(area: &Rect) -> Rect {
    Block::new()
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
        .inner(skirmish_layout(*area)[1])
        .inner(Margin {
            horizontal: 1,
            vertical: 1,
        })
}
