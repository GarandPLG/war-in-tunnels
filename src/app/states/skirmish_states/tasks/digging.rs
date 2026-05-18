use std::collections::VecDeque;

use crate::app::states::skirmish_states::tasks::Task;

#[derive(Debug, Clone, PartialEq)]
pub struct DiggingTask {
    path: VecDeque<(usize, usize)>,
}

impl DiggingTask {
    pub fn new(path: VecDeque<(usize, usize)>) -> Self {
        Self { path }
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
}
