use crate::app::states::skirmish_states::Players;
use ratatui::{
    style::{Color, Stylize},
    text::Line,
};

pub trait Structure {
    fn get_tag(&self) -> char;

    fn get_color(&self) -> Color;

    fn get_level(&self) -> char;

    fn get_stress(&self) -> u8;

    fn get_durability(&self) -> u16;

    fn get_max_durability(&self) -> u16;

    fn get_name(&self) -> &'static str;

    fn get_owner(&self) -> Players;

    fn base_text(&self) -> Vec<Line<'_>> {
        let durability: u16 = self.get_durability();
        let max_durability: u16 = self.get_max_durability();
        let durability_percent: u32 = durability as u32 * 100 / max_durability as u32;
        let stress: u8 = self.get_stress();
        let level: char = self.get_level();

        let mut lines: Vec<Line<'_>> = vec![
            Line::from_iter([
                "Durability: ".fg(Color::Rgb(192, 192, 192)),
                durability.to_string().fg(Color::Rgb(0, 128, 128)),
                "/".fg(Color::Rgb(192, 192, 192)),
                max_durability.to_string().fg(Color::Rgb(0, 128, 128)),
                " ( ".fg(Color::Rgb(192, 192, 192)),
                durability_percent.to_string().fg(Color::Rgb(0, 128, 128)),
                "% )".fg(Color::Rgb(192, 192, 192)),
            ]),
            Line::from_iter([
                "Stress: ".fg(Color::Rgb(192, 192, 192)),
                stress.to_string().fg(Color::Rgb(0, 128, 128)),
                "%".fg(Color::Rgb(192, 192, 192)),
            ]),
        ];

        if level != ' ' {
            lines.push(Line::from_iter([
                "Level: ".fg(Color::Rgb(192, 192, 192)),
                level.to_string().fg(Color::Rgb(0, 128, 128)),
            ]));
        }

        lines
    }

    fn text(&self) -> Vec<Line<'_>> {
        self.base_text()
    }
}
