use crate::app::{
    states::skirmish_states::tasks::{DiggingTask, Task},
    threads::TICK_MS,
};
use std::collections::VecDeque;

const MOVEMENT_TICKS: u16 = 3000_u16.div_ceil(TICK_MS as u16);

#[derive(Debug, Clone, PartialEq)]
pub struct MovingTask {
    path: VecDeque<(usize, usize)>,
    ticks: u16,
    total_steps: usize,
}

impl MovingTask {
    pub fn new(path: VecDeque<(usize, usize)>) -> Self {
        Self {
            total_steps: path.len(),
            path,
            ticks: 0,
        }
    }

    pub fn from_digging(task: DiggingTask) -> Self {
        Self {
            total_steps: task.get_total_steps(),
            path: task.get_path().clone(),
            ticks: 0,
        }
    }

    pub fn pop_front(&mut self) {
        self.path.pop_front();
        self.ticks = 0;
    }

    pub fn get_movement_progress(&self) -> Option<u16> {
        (self.ticks > 0).then(|| self.ticks * 100 / MOVEMENT_TICKS)
    }
}

impl Task for MovingTask {
    fn get_icon(&self) -> char {
        '→'
    }

    fn get_path(&self) -> &VecDeque<(usize, usize)> {
        &self.path
    }

    fn get_path_front_coords(&self) -> &(usize, usize) {
        self.path.front().unwrap_or(&(0, 0))
    }

    fn get_path_back_coords(&self) -> &(usize, usize) {
        self.path.back().unwrap_or(&(0, 0))
    }

    fn get_progress(&self) -> f32 {
        f32::from(self.ticks) * 100.0 / f32::from(MOVEMENT_TICKS)
    }

    fn get_total_steps(&self) -> usize {
        self.total_steps
    }

    fn progress(&mut self) {
        self.ticks = (self.ticks + 1).min(MOVEMENT_TICKS);
    }

    fn is_finished(&self) -> bool {
        self.ticks == MOVEMENT_TICKS
    }
}
