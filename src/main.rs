use ratatui::{Terminal, crossterm::event::KeyEvent, prelude::CrosstermBackend};
use std::{
    io::{Result, Stdout},
    mem::replace,
    sync::mpsc::{Receiver, Sender, channel},
    thread::{self, JoinHandle},
};
use war_in_tunnels::{
    app::{
        App,
        threads::{AppChannels, AudioCmd, handle_audio, handle_ct_events, handle_tick_event},
    },
    cli::{Cli, get_args},
};

const TICK_MS: u8 = 33;

/// Starts the terminal UI application.
///
/// The function follows the steps outlined in the module‑level documentation.
/// It returns any `std::io::Error` encountered while initializing or restoring the
/// terminal, or while running the `App`.
fn main() -> Result<()> {
    let args: Cli = get_args();

    let mut channels: AppChannels = AppChannels::new();

    let input_tx: Sender<KeyEvent> = channels.input_tx.clone();
    let resize_tx: Sender<(u16, u16)> = channels.resize_tx.clone();
    let events_thread: JoinHandle<()> =
        thread::spawn(move || handle_ct_events(input_tx, resize_tx));

    let tick_tx: Sender<()> = channels.tick_tx.clone();
    let tick_thread: JoinHandle<()> = thread::spawn(move || handle_tick_event(tick_tx, TICK_MS));

    let audio_rx: Receiver<AudioCmd> = replace(&mut channels.audio_rx, channel().1);
    // let audio_thread: JoinHandle<()> =
    thread::spawn(move || {
        handle_audio(audio_rx, args.mute, args.sound_track);
    });

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    let mut app: App = App::new(args, channels.audio_tx.clone());

    let app_result: Result<()> = app.run(&mut terminal, channels);

    ratatui::restore();

    let _ = events_thread.join();
    let _ = tick_thread.join();
    // let _ = audio_thread.join(); // TODO: kill playing music

    app_result
}
