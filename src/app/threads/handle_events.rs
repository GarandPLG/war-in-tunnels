use crate::app::threads::AppEvent;
use ratatui::crossterm::event::{Event, read};
use std::sync::mpsc::Sender;

/// Reads *all* crossterm events and forwards the ones we care about.
pub fn handle_events(tx: Sender<AppEvent>) {
    loop {
        match read() {
            Ok(ev) => match ev {
                Event::Key(key) => {
                    if tx.send(AppEvent::Input(key)).is_err() {
                        break;
                    }
                }
                Event::Resize(cols, rows) => {
                    if tx.send(AppEvent::Resize(cols, rows)).is_err() {
                        break;
                    }
                }
                _ => {}
            },
            Err(_) => continue,
        }
    }
}
