use ratatui::crossterm::event::KeyEvent;

pub enum AppEvent {
    Input(KeyEvent),
    Resize(u16, u16),
}
