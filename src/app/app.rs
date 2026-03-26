use crate::{
    app::{GameStates, handle_keybindings, view::View},
    cli::Cli,
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyEvent},
};
use std::{
    io::Result,
    sync::mpsc::{self, Receiver, RecvTimeoutError},
    time::Duration,
};

pub enum Event {
    Input(KeyEvent),
    Resize(u16, u16),
}

pub struct App {
    pub exit: bool,
    pub view: View,
    pub states: GameStates,
}

impl App {
    pub fn new(args: Cli) -> Self {
        Self {
            exit: false,
            view: args.view,
            states: GameStates::new(args),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw(frame))?;

            let event = match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(ev) => ev,
                Err(RecvTimeoutError::Timeout) => {
                    continue;
                }
                Err(_) => break,
            };
            match event {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
                Event::Resize(_, _) => {}
            }
        }

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame<'_>) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        handle_keybindings(self, key_event);
        Ok(())
    }
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match event::read() {
            Ok(ev) => match ev {
                event::Event::Key(key_event) => {
                    if tx.send(Event::Input(key_event)).is_err() {
                        break;
                    }
                }
                event::Event::Resize(cols, rows) => {
                    if tx.send(Event::Resize(cols, rows)).is_err() {
                        break;
                    }
                }
                _ => {}
            },
            Err(_) => {
                continue;
            }
        }
    }
}
