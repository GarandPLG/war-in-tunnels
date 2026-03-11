use crate::app::{App, View, widgets::keybindings_widget};
use clap::ValueEnum;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

fn view_options() -> Vec<(usize, String)> {
    View::value_variants()
        .iter()
        .enumerate()
        .map(|(i, v)| {
            let name = v
                .to_possible_value()
                .unwrap()
                .get_name()
                .replace('-', " ")
                .to_uppercase()
                .to_string();
            (i, name)
        })
        .filter(|(_, v)| v != "MAIN MENU")
        .collect()
}

pub fn main_menu_widget(app: &App, area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout =
        Layout::vertical([Constraint::Percentage(88), Constraint::Percentage(12)]);

    let [main_menu_area, keybindings_area] = vertical_layout.areas(area);

    Block::new()
        .borders(Borders::LEFT | Borders::TOP | Borders::RIGHT)
        .render(main_menu_area, buf);

    let main_menu_areas: Vec<Rect> = main_menu_area.layout_vec(&Layout::vertical([
        Constraint::Percentage(50),
        Constraint::Percentage(50),
    ]));

    {
        let title_area: Rect = main_menu_areas[0].centered_vertically(Constraint::Percentage(50));

        let title_text: String = vec![
            r" __    __             _         _____                        _     ",
            r"/ / /\ \ \__ _ _ __  (_)_ __   /__   \_   _ _ __  _ __   ___| |___ ",
            r"\ \/  \/ / _` | '__| | | '_ \    / /\/ | | | '_ \| '_ \ / _ \ / __|",
            r" \  /\  / (_| | |    | | | | |  / /  | |_| | | | | | | |  __/ \__ \",
            r"  \/  \/ \__,_|_|    |_|_| |_|  \/    \__,_|_| |_|_| |_|\___|_|___/",
        ]
        .join("\n");

        Paragraph::new(title_text)
            .alignment(Alignment::Center)
            .yellow()
            .block(Block::new().gray().borders(Borders::LEFT | Borders::RIGHT))
            .render(title_area, buf);
    }

    {
        let options_area: Rect = main_menu_areas[1];

        let lines: Vec<Line<'_>> = view_options()
            .into_iter()
            .map(|(i, view)| {
                let styled = if app.game_states.main_menu_state.selected_view == i {
                    Line::from(format!("> {}", view)).yellow()
                } else {
                    Line::from(view).white()
                };
                styled
            })
            .collect();

        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .green()
            .block(Block::new().gray().borders(Borders::LEFT | Borders::RIGHT))
            .render(options_area, buf);
    }

    keybindings_widget(View::MainMenu).render(keybindings_area, buf);
}
