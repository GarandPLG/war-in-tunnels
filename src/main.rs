use ratatui::{Terminal, crossterm::event::KeyEvent, prelude::CrosstermBackend};
use std::{
    io::{Result, Stdout},
    sync::mpsc::channel,
    thread,
};
use war_in_tunnels::{
    app::{
        App,
        threads::{AudioCmd, handle_audio, handle_ct_events, handle_tick_event},
    },
    cli::{Cli, get_args},
};

/// Starts the terminal UI application.
///
/// The function follows the steps outlined in the module‑level documentation.
/// It returns any `std::io::Error` encountered while initializing or restoring the
/// terminal, or while running the `App`.
fn main() -> Result<()> {
    let args: Cli = get_args();

    let (input_tx, input_rx) = channel::<KeyEvent>();
    let (resize_tx, resize_rx) = channel::<(u16, u16)>();
    let (tick_tx, tick_rx) = channel::<()>();
    let (audio_tx, audio_rx) = channel::<AudioCmd>();

    thread::spawn(move || handle_ct_events(input_tx, resize_tx));
    thread::spawn(move || handle_tick_event(tick_tx));
    if !args.disable_audio {
        thread::spawn(move || handle_audio(audio_rx, args.mute, args.soundtrack));
    }

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    let mut app: App = App::new(args, audio_tx);

    let app_result: Result<()> = app.run(&mut terminal, input_rx, resize_rx, tick_rx);

    ratatui::restore();

    app_result
}
