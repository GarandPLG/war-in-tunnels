use crate::app::{
  App, View,
  keybindings::{Action, event_to_action},
  threads::AudioCmd,
};
use ratatui::crossterm::event::KeyEvent;

pub fn common_keybindings(app: &mut App, action: Action) {
  match action {
    Action::Quit | Action::Quit2 => app.exit = true,
    Action::Esc => app.view = View::MainMenu,
    Action::Mute => if let Err(_) = app.audio_tx.send(AudioCmd::Mute) {},
    Action::VolumeUp => if let Err(_) = app.audio_tx.send(AudioCmd::VolumeUp) {},
    Action::VolumeDown => if let Err(_) = app.audio_tx.send(AudioCmd::VolumeDown) {},
    _ => (),
  }
}

pub fn default_keybindings(app: &mut App, key_event: &KeyEvent) {
  if let Some(action) = event_to_action(&key_event) {
    common_keybindings(app, action);
  }
}
