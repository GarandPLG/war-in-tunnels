use ratatui::{Terminal, prelude::CrosstermBackend};
use std::{
    io::{Result, Stdout},
    sync::mpsc::{self, Sender},
    thread,
};
use war_in_tunnels::{
    app::{App, Event, handle_input_events},
    cli::{Cli, get_args},
};

fn main() -> Result<()> {
    let args: Cli = get_args();
    let mut terminal: Terminal<CrosstermBackend<Stdout>> = ratatui::init();
    let mut app: App = App::new(args);

    let (event_tx, event_rx) = mpsc::channel::<Event>();

    let tx_to_input_events: Sender<Event> = event_tx.clone();
    thread::spawn(move || {
        handle_input_events(tx_to_input_events);
    });

    let app_result: Result<()> = app.run(&mut terminal, event_rx);
    ratatui::restore();

    app_result
}
