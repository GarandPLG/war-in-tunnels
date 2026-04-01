#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    value: usize,
    max: usize,
}

impl Offset {
    pub fn new(initial_value: Option<usize>, initial_max: Option<usize>) -> Self {
        Self {
            value: initial_value.unwrap_or(0),
            max: initial_max.unwrap_or(0),
        }
    }

    pub fn get_value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }

    pub fn set_max(&mut self, max: usize) {
        self.max = max;
    }

    pub fn next(&mut self) {
        self.value = self.value.saturating_add(1).min(self.max);
    }

    pub fn prev(&mut self) {
        self.value = self.value.saturating_sub(1).max(0)
    }
}
