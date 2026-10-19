use crate::app::states::skirmish_states::tasks::{MovingTask, Task};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct DiggingTask {
  path: VecDeque<(usize, usize)>,
  progress: f32,
  tick_increase: f32,
  total_steps: usize,
}

impl DiggingTask {
  pub fn new(path: VecDeque<(usize, usize)>, tick_increase: f32) -> Self {
    Self {
      total_steps: path.len(),
      path,
      progress: 0.0,
      tick_increase,
    }
  }

  pub fn pop_front(&mut self) {
    self.path.pop_front();
    self.progress = 0.0;
  }

  pub fn from_moving(task: MovingTask, tick_increase: f32) -> Self {
    Self {
      path: task.get_path().clone(),
      progress: 0.0,
      tick_increase,
      total_steps: task.get_total_steps(),
    }
  }

  pub fn current_target(&self) -> Option<(usize, usize)> {
    self.path.front().copied()
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

  fn get_total_steps(&self) -> usize {
    self.total_steps
  }

  fn progress(&mut self) {
    self.progress = (self.progress + self.tick_increase).clamp(0.0, 100.0);
  }

  fn is_finished(&self) -> bool {
    self.progress >= 100.0
  }
}
