use clap::ValueEnum;
use ratatui::widgets::ScrollbarState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub vertical_scrollbar: ScrollbarState,
    pub horizontal_scrollbar: ScrollbarState,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}
