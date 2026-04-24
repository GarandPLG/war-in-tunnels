use crate::app::views::{skirmish_layout, skirmish_main_area_layout};
use ratatui::{
    layout::{Margin, Rect},
    widgets::{Block, Borders},
};

pub fn cells_area_helper(area: &Rect, side_panel: bool) -> Rect {
    let board_area: Rect = skirmish_main_area_layout(skirmish_layout(*area)[1], side_panel)[0];

    Block::new()
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
        .inner(board_area)
        .inner(Margin {
            horizontal: 1,
            vertical: 1,
        })
}
