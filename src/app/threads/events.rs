use ratatui::crossterm::event::{Event, KeyEvent, read};
use std::sync::mpsc::Sender;

pub fn handle_ct_events(input_tx: Sender<KeyEvent>, resize_tx: Sender<(u16, u16)>) {
    loop {
        match read() {
            Ok(Event::Key(k)) => {
                if input_tx.send(k).is_err() {
                    break;
                }
            }
            Ok(Event::Resize(cols, rows)) => {
                if resize_tx.send((cols, rows)).is_err() {
                    break;
                }
            }
            Ok(_) => {}
            Err(_) => continue,
        }
    }
}
