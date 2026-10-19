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
                "Durability: ".gray(),
                durability.to_string().cyan(),
                "/".gray(),
                max_durability.to_string().cyan(),
                " ( ".gray(),
                durability_percent.to_string().cyan(),
                "% )".gray(),
            ]),
            Line::from_iter(["Stress: ".gray(), stress.to_string().cyan(), "%".gray()]),
        ];

        if level != ' ' {
            lines.push(Line::from_iter([
                "Level: ".gray(),
                level.to_string().cyan(),
            ]));
        }

        lines
    }

    fn text(&self) -> Vec<Line<'_>> {
        self.base_text()
    }
}
