use crate::app::{
    App, View,
    helpers::main_menu_option_helper,
    keybindings::{Action, count_largest_group},
    widgets::KeybindingsWidget,
};
use clap::ValueEnum;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

const ACTIONS: &[Action] = &[
    Action::Up,
    Action::Down,
    Action::Space,
    Action::VolumeUp,
    Action::VolumeDown,
    Action::Mute,
    Action::Quit,
    Action::Quit2,
];

fn main_menu_layout(area: Rect) -> [Rect; 2] {
    Layout::vertical([
        Constraint::Fill(1),
        Constraint::Length(count_largest_group(&ACTIONS) + 2),
    ])
    .areas(area)
}

pub fn main_menu_view(app: &App, area: Rect, buf: &mut Buffer) {
    let Some(states) = app.states_ref() else {
        return;
    };

    let [main_menu_area, keybindings_area] = main_menu_layout(area);

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
            .fg(Color::Rgb(128, 128, 0))
            .block(
                Block::new()
                    .fg(Color::Rgb(192, 192, 192))
                    .borders(Borders::LEFT | Borders::RIGHT),
            )
            .render(title_area, buf);
    }

    {
        let options_area: Rect = main_menu_areas[1];

        let lines: Vec<Line<'_>> = View::value_variants()
            .iter()
            .enumerate()
            .filter(|(_, v)| v != &&View::MainMenu)
            .map(|(i, view)| {
                let view_string: String = main_menu_option_helper(format!("{:?}", view));

                let styled: Line<'_> = if states.main_menu.selected_view == i {
                    Line::from_iter([
                        "> ".fg(Color::Rgb(0, 128, 128)),
                        view_string.fg(Color::Rgb(128, 128, 0)),
                    ])
                    .fg(Color::Rgb(128, 128, 0))
                } else {
                    Line::from(format!("  {view_string}")).fg(Color::Rgb(255, 255, 255))
                };

                styled
            })
            .collect();

        Paragraph::new(lines)
            .alignment(Alignment::Center)
            .fg(Color::Rgb(0, 128, 0))
            .block(
                Block::new()
                    .fg(Color::Rgb(192, 192, 192))
                    .borders(Borders::LEFT | Borders::RIGHT),
            )
            .render(options_area, buf);
    }

    {
        KeybindingsWidget::new(ACTIONS, keybindings_area.height).render(keybindings_area, buf);
    }
}
