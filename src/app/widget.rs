use crate::app::{App, View, widgets::main_menu_widget};
use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.view {
            View::MainMenu => main_menu_widget(self, area, buf),
            _ => (),
        }
    }
}
