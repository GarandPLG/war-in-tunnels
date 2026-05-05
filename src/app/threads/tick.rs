use std::{
    sync::mpsc::Sender,
    thread,
    time::{Duration, Instant},
};

pub const TICK_MS: u8 = 33;

pub fn handle_tick_event(tx: Sender<()>) {
    let interval: Duration = Duration::from_millis(TICK_MS as u64);
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
