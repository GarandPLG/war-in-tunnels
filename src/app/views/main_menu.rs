use crate::app::{App, View, keybindings::Action, widgets::KeybindingsWidget};
use clap::ValueEnum;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

fn format_view_string(s: String) -> String {
    if let Some(pos) = s
        .char_indices()
        .filter(|(_, c)| c.is_ascii_uppercase())
        .nth(1)
        .map(|(i, _)| i)
    {
        let (first, second) = s.split_at(pos);
        format!("{first} {second}")
    } else {
        s
    }
}

pub fn main_menu_view(app: &App, area: Rect, buf: &mut Buffer) {
    let vertical_layout: Layout = Layout::vertical([Constraint::Fill(1), Constraint::Length(4)]);

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

        let lines: Vec<Line<'_>> = View::value_variants()
            .iter()
            .enumerate()
            .filter(|(_, v)| v != &&View::MainMenu)
            .map(|(i, view)| {
                let view_string: String = format_view_string(format!("{:?}", view));

                let styled: Line<'_> = if app.game_states.main_menu_state.selected_view == i {
                    Line::from(format!("> {view_string}")).yellow()
                } else {
                    Line::from(view_string).white()
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

    {
        let actions: Vec<Action> = vec![
            Action::Up,
            Action::Down,
            Action::Space,
            Action::Quit,
            Action::Quit2,
        ];

        KeybindingsWidget::new(actions).render(keybindings_area, buf);
    }
}
