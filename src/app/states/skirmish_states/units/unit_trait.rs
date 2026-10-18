use crate::app::states::skirmish_states::{Players, tasks::Tasks};
use ratatui::{
    style::Stylize,
    text::{Line, Span},
};
use std::collections::VecDeque;

pub trait Unit {
    fn is_unit(&self) -> bool {
        true
    }

    fn get_owner(&self) -> Players;

    fn get_tag(&self) -> char;

    fn get_name(&self) -> &'static str;

    fn get_hp(&self) -> u16;

    fn get_max_hp(&self) -> u16;

    fn get_can_dig(&self) -> bool;

    fn get_digging_power(&self) -> f32;

    fn get_coords(&self) -> (usize, usize);

    fn set_coords(&mut self, coords: (usize, usize));

    fn get_ref_tasks(&self) -> &VecDeque<Tasks>;

    fn get_mut_tasks(&mut self) -> &mut VecDeque<Tasks>;

    fn set_task(&mut self, task: Tasks);

    fn get_tasks_formatted(&self) -> Vec<Line<'_>>;

    fn base_text(&self) -> Vec<Line<'_>> {
        let owner: Span<'_> = self.get_owner().get_span();
        let hp: u16 = self.get_hp();
        let max_hp: u16 = self.get_max_hp();
        let hp_percent: u16 = hp * 100 / max_hp;
        let can_dig: bool = self.get_can_dig();
        let digging_power: f32 = self.get_digging_power();
        let have_any_tasks: bool = self.get_ref_tasks().is_empty();
        let tasks: Vec<Line<'_>> = self.get_tasks_formatted();

        let mut lines: Vec<Line<'_>> = Vec::from([
            Line::from_iter(["Owner: ".gray(), owner]),
            Line::from_iter([
                "HP: ".gray(),
                hp.green(),
                "/".gray(),
                max_hp.green(),
                " ( ".gray(),
                hp_percent.green(),
                "% )".gray(),
            ]),
        ]);

        if can_dig {
            lines.push(Line::from_iter([
                "Digging power: ".gray(),
                digging_power.green(),
            ]));
        }

        if !have_any_tasks {
            lines.push(Line::from("Tasks Queue: ".gray()));
            for task in tasks {
                lines.push(task);
            }
        }

        lines
    }

    fn text(&self) -> Vec<Line<'_>> {
        self.base_text()
    }
}
