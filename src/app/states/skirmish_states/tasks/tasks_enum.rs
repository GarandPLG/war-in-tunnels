use crate::app::states::skirmish_states::tasks::{DiggingTask, MovingTask, Task};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Tasks {
    Digging(DiggingTask),
    Moving(MovingTask),
}

impl Tasks {
    #[inline]
    fn t(&self) -> &dyn Task {
        match self {
            Self::Digging(d) => d,
            Self::Moving(m) => m,
        }
    }

    #[inline]
    fn t_mut(&mut self) -> &mut dyn Task {
        match self {
            Self::Digging(d) => d,
            Self::Moving(m) => m,
        }
    }
}

impl Task for Tasks {
    fn get_icon(&self) -> char {
        self.t().get_icon()
    }

    fn get_path(&self) -> &VecDeque<(usize, usize)> {
        self.t().get_path()
    }

    fn get_path_front_coords(&self) -> &(usize, usize) {
        self.t().get_path_front_coords()
    }

    fn get_path_back_coords(&self) -> &(usize, usize) {
        self.t().get_path_back_coords()
    }

    fn get_progress(&self) -> f32 {
        self.t().get_progress()
    }

    fn get_total_steps(&self) -> usize {
        self.t().get_total_steps()
    }

    fn is_finished(&self) -> bool {
        self.t().is_finished()
    }

    fn progress(&mut self) {
        self.t_mut().progress();
    }
}
