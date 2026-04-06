use crate::app::threads::AppEvent;
use ratatui::crossterm::event::{Event, read};
use std::sync::mpsc::Sender;

pub fn handle_input_events(tx: Sender<AppEvent>) {
    loop {
        match read() {
            Ok(ev) => match ev {
                Event::Key(key) => {
                    if tx.send(AppEvent::Input(key)).is_err() {
                        break;
                    }
                }
                Event::Resize(c, r) => {
                    if tx.send(AppEvent::Resize(c, r)).is_err() {
                        break;
                    }
                }
                _ => {}
            },
            Err(_) => continue,
        }
    }
}
