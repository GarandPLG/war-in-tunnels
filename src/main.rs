use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{
    io::{Result, Stdout},
    sync::mpsc::channel,
    thread::{self, JoinHandle},
};
use war_in_tunnels::{
    app::{
        App,
        threads::{AppEvent, handle_events},
    },
    cli::{Cli, get_args},
    logs::init_logger,
};

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

    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    let mut app: App = App::new(args);

    let (app_event_tx, app_event_rx) = channel::<AppEvent>();

    let app_event_thread: JoinHandle<()> = thread::spawn(move || {
        handle_events(app_event_tx);
    });

    let app_result: Result<()> = app.run(&mut terminal, app_event_rx);
    ratatui::restore();

    let _ = app_event_thread.join();

    app_result
}
