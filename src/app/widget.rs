use crate::app::{
    App, View,
    widgets::{default_view_widget, main_menu_widget},
};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.view {
            View::MainMenu => main_menu_widget(self, area, buf),
            _ => default_view_widget(self, area, buf),
        }
    }
}
