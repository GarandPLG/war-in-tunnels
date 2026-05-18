use crate::app::states::skirmish_states::tasks::{DiggingTask, Task};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub enum Tasks {
    Digging(DiggingTask),
}

impl Tasks {
    #[inline]
    fn t(&self) -> &dyn Task {
        match self {
            Self::Digging(d) => d,
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
}
