use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
pub struct DiggingTask {
    path: VecDeque<(usize, usize)>,
}

impl DiggingTask {
    pub fn new(path: VecDeque<(usize, usize)>) -> Self {
        Self { path }
    }
}
