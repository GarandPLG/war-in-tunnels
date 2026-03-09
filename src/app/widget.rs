use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::{App, widgets::main_menu_widget};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        if self.default_window == "main_menu" {
            main_menu_widget(area, buf);
        }
    }
}
