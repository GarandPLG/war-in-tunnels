use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{
    io::{Result, Stdout},
    sync::mpsc::channel,
    thread::{
        self,
        // JoinHandle
    },
};
use war_in_tunnels::{
    app::{
        App,
        threads::{AppEvent, AudioCmd, handle_audio, handle_events, spawn_tick_thread},
    },
    cli::{Cli, get_args},
    logs::init_logger,
};

const TICK_MS: u64 = 33;

/// Starts the terminal UI application.
///
/// The function follows the steps outlined in the module‑level documentation.
/// It returns any `std::io::Error` encountered while initializing or restoring the
/// terminal, or while running the `App`.
fn main() -> Result<()> {
    let args: Cli = get_args();
    if args.log {
        init_logger();
    }

    let (app_event_tx, app_event_rx) = channel::<AppEvent>();

    // let app_event_thread: JoinHandle<()> = thread::spawn(move || {
    //     handle_events(app_event_tx);
    // });

    thread::spawn(move || {
        handle_events(app_event_tx.clone());
    });

    thread::spawn(move || {
        spawn_tick_thread(app_event_tx, TICK_MS);
    });

    let (audio_tx, audio_rx) = channel::<AudioCmd>();

    // let audio_event_thread: JoinHandle<()> = thread::spawn(move || {
    //     handle_audio(audio_rx);
    // });

    thread::spawn(move || {
        handle_audio(audio_rx, args.mute, args.sound_track);
    });

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    let mut app: App = App::new(args, audio_tx);

    let app_result: Result<()> = app.run(&mut terminal, app_event_rx);

    ratatui::restore();

    // let _ = app_event_thread.;
    // let _ = audio_event_thread.join();

    app_result
}
