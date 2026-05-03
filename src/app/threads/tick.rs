use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

pub fn handle_tick_event(tx: Sender<()>, interval_ms: u8) {
    let interval: Duration = Duration::from_millis(interval_ms as u64);
    loop {
        if tx.send(()).is_err() {
            break;
        }

        let elapsed: Duration = Instant::now().elapsed();
        if interval > elapsed {
            thread::sleep(interval - elapsed);
        }
    }
}
