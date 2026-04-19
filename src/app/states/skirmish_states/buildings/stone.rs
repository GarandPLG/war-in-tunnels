use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stone {
    durability: u16,
}

impl Stone {
    pub fn new() -> Self {
        Self { durability: 1000 }
    }

    pub fn get_tag(&self) -> char {
        ' '
    }

    pub fn get_color(&self) -> Color {
        Color::White
    }
}
