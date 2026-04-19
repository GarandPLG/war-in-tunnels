use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tunnel {
    durability: u16,
    stress: u8,
    roof_support: bool,
    rail: bool,
    lamp: bool,
}

impl Tunnel {
    pub fn new() -> Self {
        Self {
            durability: 500,
            stress: 25,
            roof_support: false,
            rail: false,
            lamp: false,
        }
    }

    pub fn get_tag(&self) -> char {
        'T'
    }

    pub fn get_color(&self) -> Color {
        Color::Gray
    }

    pub fn get_durability(&self) -> u16 {
        self.durability
    }

    pub fn get_stress(&self) -> u8 {
        self.stress
    }
}
