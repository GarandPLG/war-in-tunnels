use crate::app::{
    App, View,
    widgets::{KeybindingsWidget, main_menu_widget},
};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Paragraph, Widget},
};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self.view {
            View::MainMenu => main_menu_widget(self, area, buf),
            _ => {
                let vertical_layout: Layout =
                    Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]);

                let [main_area, keybindings_area] = vertical_layout.areas(area);

                Paragraph::new("Work in progress")
                    .alignment(Alignment::Center)
                    .yellow()
                    .block(
                        Block::new()
                            .gray()
                            .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT),
                    )
                    .render(main_area, buf);

                KeybindingsWidget::new(self.view).render(keybindings_area, buf);
            }
        }
    }
}
