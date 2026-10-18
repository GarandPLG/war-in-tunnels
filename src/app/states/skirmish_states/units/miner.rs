use ratatui::{style::Stylize, text::Line};

use crate::app::states::skirmish_states::{
    Players,
    tasks::{Task, Tasks},
    units::Unit,
};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct MinerUnit {
    owner: Players,
    hp: u16,
    can_dig: bool,
    digging_power: f32,
    coords: (usize, usize),
    tasks: VecDeque<Tasks>,
}

impl MinerUnit {
    pub fn new(owner: Players, coords: (usize, usize)) -> Self {
        Self {
            owner,
            hp: 100,
            can_dig: true,
            digging_power: 0.55,
            coords,
            tasks: VecDeque::new(),
        }
    }

    fn col_to_letters(&self, col: usize) -> String {
        let mut col: usize = col + 1;
        let mut letters: Vec<char> = Vec::new();

        while col > 0 {
            letters.push((b'A' + ((col - 1) % 26) as u8) as char);
            col = (col - 1) / 26;
        }

        letters.iter().rev().collect()
    }

    fn display_coords(&self, coords: &(usize, usize)) -> String {
        format!("{}{}", self.col_to_letters(coords.0), coords.1)
    }
}

impl Unit for MinerUnit {
    fn get_owner(&self) -> Players {
        self.owner
    }

    fn get_tag(&self) -> char {
        'M'
    }

    fn get_name(&self) -> &'static str {
        "Miner"
    }

    fn get_hp(&self) -> u16 {
        self.hp
    }

    fn get_max_hp(&self) -> u16 {
        100
    }

    fn get_can_dig(&self) -> bool {
        self.can_dig
    }

    fn get_digging_power(&self) -> f32 {
        self.digging_power
    }

    fn get_coords(&self) -> (usize, usize) {
        self.coords
    }

    fn set_coords(&mut self, coords: (usize, usize)) {
        self.coords = coords;
    }

    fn get_ref_tasks(&self) -> &VecDeque<Tasks> {
        &self.tasks
    }

    fn get_mut_tasks(&mut self) -> &mut VecDeque<Tasks> {
        &mut self.tasks
    }

    fn get_tasks_formatted(&self) -> Vec<Line<'_>> {
        Vec::from(
            self.tasks
                .iter()
                .enumerate()
                .flat_map(|(i, task): (usize, &Tasks)| -> [Line<'_>; 2] {
                    [
                        Line::from_iter([
                            format!("{}. ", i + 1).gray(),
                            task.get_icon().gray(),
                            "  ".gray(),
                            self.display_coords(task.get_path_front_coords()).green(),
                            " -> ".gray(),
                            self.display_coords(task.get_path_back_coords()).yellow(),
                        ]),
                        Line::from(format!(
                            "   Step: {}% | Total: {}%",
                            task.get_progress() as u16,
                            task.get_total_progress() as u16,
                        ))
                        .cyan(),
                    ]
                })
                .collect::<Vec<Line<'_>>>(),
        )
    }

    fn set_task(&mut self, task: Tasks) {
        self.tasks.push_back(task);
    }
}
