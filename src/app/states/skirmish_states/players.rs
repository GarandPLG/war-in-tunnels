use ratatui::{
  style::{Color, Stylize},
  text::Span,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Players {
  Player,
  Enemy,
  Unclaimed,
}

impl Players {
  pub fn get_text(&self) -> &'static str {
    match self {
      Self::Player => "Player",
      Self::Enemy => "Enemy",
      Self::Unclaimed => "Unclaimed",
    }
  }

  pub fn get_color(&self) -> Color {
    match self {
      Self::Player => Color::LightBlue,
      Self::Enemy => Color::LightRed,
      Self::Unclaimed => Color::White,
    }
  }

  pub fn get_span(&self) -> Span<'static> {
    match self {
      Self::Player => self.get_text().fg(self.get_color()),
      Self::Enemy => self.get_text().fg(self.get_color()),
      Self::Unclaimed => self.get_text().fg(self.get_color()),
    }
  }
}
