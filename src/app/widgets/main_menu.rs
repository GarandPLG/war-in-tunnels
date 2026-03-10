use crate::app::{View, widgets::keybindings_widget};
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Color,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn main_menu_widget(area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout =
        Layout::vertical([Constraint::Percentage(87), Constraint::Percentage(13)]);

    let [main_menu_area, keybindings_area] = vertical_layout.areas(area);

    {
        let title_text: String = vec![
            r" __    __             _         _____                        _     ",
            r"/ / /\ \ \__ _ _ __  (_)_ __   /__   \_   _ _ __  _ __   ___| |___",
            r"\ \/  \/ / _` | '__| | | '_ \    / /\/ | | | '_ \| '_ \ / _ \ / __|",
            r" \  /\  / (_| | |    | | | | |  / /  | |_| | | | | | | |  __/ \__ \",
            r"  \/  \/ \__,_|_|    |_|_| |_|  \/    \__,_|_| |_|_| |_|\___|_|___/",
        ]
        .join("\n");

        let title: Paragraph<'_> = Paragraph::new(title_text)
            .alignment(Alignment::Center)
            .style(Color::Yellow)
            .block(
                Block::default()
                    .borders(Borders::LEFT | Borders::RIGHT | Borders::TOP)
                    .style(Color::Gray),
            );

        title.render(main_menu_area, buf);
    }

    {
        let keybindings: Paragraph<'_> = keybindings_widget(View::MainMenu);
        keybindings.render(keybindings_area, buf);
    }
}
