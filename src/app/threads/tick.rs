use crate::app::threads::AppEvent;
use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

pub fn spawn_tick_thread(tx: Sender<AppEvent>, interval_ms: u64) {
    thread::spawn(move || {
        let interval: Duration = Duration::from_millis(interval_ms);
        loop {
            if tx.send(AppEvent::Tick).is_err() {
                break;
            }

            let elapsed: Duration = Instant::now().elapsed();
            if interval > elapsed {
                thread::sleep(interval - elapsed);
            }
        }
    });
}
