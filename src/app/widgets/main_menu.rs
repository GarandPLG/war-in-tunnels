use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

pub fn main_menu_widget(area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout =
        Layout::vertical([Constraint::Percentage(90), Constraint::Percentage(10)]);

    let [main_menu_area, keybindings_area] = vertical_layout.areas(area);

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

    let keybindings_text: Vec<Line<'_>> =
        vec![Line::from_iter(["q".bold().red(), "\t - Quit".into()])];

    let keybindings: Paragraph<'_> = Paragraph::new(keybindings_text)
        .alignment(Alignment::Left)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("[ Keybinding ]"),
        );

    keybindings.render(keybindings_area, buf);
}
