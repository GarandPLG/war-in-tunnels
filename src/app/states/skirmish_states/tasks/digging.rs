use crate::app::states::skirmish_states::tasks::Task;
use std::{collections::VecDeque, ops::Add};

#[derive(Debug, Clone, PartialEq)]
pub struct DiggingTask {
    path: VecDeque<(usize, usize)>,
    progress: f32,
    tick_increase: f32,
}

impl DiggingTask {
    pub fn new(path: VecDeque<(usize, usize)>, tick_increase: f32) -> Self {
        Self {
            path,
            progress: 0.0,
            tick_increase,
        }
    }
}

impl Task for DiggingTask {
    fn get_icon(&self) -> char {
        '⛏'
    }

    fn get_path(&self) -> &VecDeque<(usize, usize)> {
        &self.path
    }

    fn get_path_front_coords(&self) -> &(usize, usize) {
        self.path.front().map_or(&(0, 0), |c| c)
    }

    fn get_path_back_coords(&self) -> &(usize, usize) {
        self.path.back().map_or(&(0, 0), |c| c)
    }

    fn get_progress(&self) -> f32 {
        self.progress
    }

    fn progress(&mut self) {
        let _ = self.progress.add(self.tick_increase).max(0.0).min(100.0);
    }

    fn is_finished(&self) -> bool {
        self.progress >= 100.0
    }
}
