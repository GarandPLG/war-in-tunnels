use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::app::{App, View, widgets::main_menu_widget};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.window {
            View::MainMenu => main_menu_widget(area, buf),
            _ => panic!("This window doesn't have widget."),
        }
    }
}
