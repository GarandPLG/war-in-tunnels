use crate::app::{
    App,
    views::{default_view, main_menu_view, settings_view},
};
use clap::ValueEnum;
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum View {
    MainMenu,
    Skirmish,
    PerkDecks,
    SkillsConfig,
    Settings,
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.view {
            View::MainMenu => main_menu_view(self, area, buf),
            View::Settings => settings_view(self, area, buf),
            _ => default_view(area, buf),
        }
    }
}
