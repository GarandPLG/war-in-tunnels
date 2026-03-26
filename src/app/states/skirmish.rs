use clap::ValueEnum;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    value: usize,
    max: usize,
}

impl Offset {
    pub fn new() -> Self {
        Self { value: 0, max: 0 }
    }

    pub fn get_value(&self) -> usize {
        self.value
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SkirmishState {
    pub id: usize,
    pub name: &'static str,
    pub vertical_offset: Offset,
    pub horizontal_offset: Offset,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    FrontLines,
}
