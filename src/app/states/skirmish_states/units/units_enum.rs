use crate::app::states::skirmish_states::{
    Players,
    tasks::Tasks,
    units::{MinerUnit, Unit},
};
use ratatui::text::Line;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Units {
    Miner(MinerUnit),
}

impl Units {
    #[inline]
    pub fn u(&self) -> &dyn Unit {
        match self {
            Self::Miner(m) => m,
        }
    }

    #[inline]
    pub fn u_mut(&mut self) -> &mut dyn Unit {
        match self {
            Self::Miner(m) => m,
        }
    }
}

impl Unit for Units {
    fn get_tag(&self) -> char {
        self.u().get_tag()
    }

    fn get_name(&self) -> &'static str {
        self.u().get_name()
    }

    fn get_owner(&self) -> Players {
        self.u().get_owner()
    }

    fn get_hp(&self) -> u16 {
        self.u().get_hp()
    }

    fn get_max_hp(&self) -> u16 {
        self.u().get_max_hp()
    }

    fn get_can_dig(&self) -> bool {
        self.u().get_can_dig()
    }

    fn get_digging_power(&self) -> f32 {
        self.u().get_digging_power()
    }

    fn get_coords(&self) -> (usize, usize) {
        self.u().get_coords()
    }

    fn get_ref_tasks(&self) -> &VecDeque<Tasks> {
        self.u().get_ref_tasks()
    }

    fn get_mut_tasks(&mut self) -> &mut VecDeque<Tasks> {
        self.u_mut().get_mut_tasks()
    }

    fn get_tasks_formatted(&self) -> Vec<Line<'_>> {
        self.u().get_tasks_formatted()
    }

    fn set_task(&mut self, task: Tasks) {
        self.u_mut().set_task(task);
    }
}

impl Unit for Option<Units> {
    fn is_unit(&self) -> bool {
        self.as_ref().map_or(false, |u| u.is_unit())
    }

    fn get_tag(&self) -> char {
        self.as_ref().map_or(' ', |u| u.get_tag())
    }

    fn get_name(&self) -> &'static str {
        self.as_ref().map_or("", |u| u.get_name())
    }

    fn get_owner(&self) -> Players {
        self.as_ref().map_or(Players::Unclaimed, |u| u.get_owner())
    }

    fn get_hp(&self) -> u16 {
        self.as_ref().map_or(0, |u| u.get_hp())
    }

    fn get_max_hp(&self) -> u16 {
        self.as_ref().map_or(0, |u| u.get_max_hp())
    }

    fn get_can_dig(&self) -> bool {
        self.as_ref().map_or(false, |u| u.get_can_dig())
    }

    fn get_digging_power(&self) -> f32 {
        self.as_ref().map_or(0.0, |u| u.get_digging_power())
    }

    fn get_coords(&self) -> (usize, usize) {
        self.as_ref().map_or((0, 0), |u| u.get_coords())
    }

    fn get_ref_tasks(&self) -> &VecDeque<Tasks> {
        self.as_ref()
            .expect("attempted to get referenced tasks from `None`")
            .get_ref_tasks()
    }

    fn get_mut_tasks(&mut self) -> &mut VecDeque<Tasks> {
        self.as_mut()
            .expect("attempted to get mutable tasks from `None`")
            .get_mut_tasks()
    }

    fn get_tasks_formatted(&self) -> Vec<Line<'_>> {
        self.as_ref()
            .map_or(Vec::new(), |u| u.get_tasks_formatted())
    }

    fn set_task(&mut self, task: Tasks) {
        self.as_mut().map_or((), |u| u.set_task(task))
    }
}
