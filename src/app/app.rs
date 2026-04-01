use crate::{
    app::{GameStates, handle_keybindings, view::View},
    cli::Cli,
};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyEvent},
    layout::Rect,
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
    pub window_area: Rect,
    pub args: Cli,
    pub states: Option<GameStates>,
}

impl App {
    pub fn new(args: Cli) -> Self {
        Self {
            exit: false,
            view: args.view,
            window_area: Rect::default(),
            args: args,
            states: None,
        }
    }

    pub fn states(&self) -> Option<&GameStates> {
        self.states.as_ref()
    }

    pub fn states_mut(&mut self) -> Option<&mut GameStates> {
        self.states.as_mut()
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
        let window_area: Rect = frame.area();

        if window_area.width != self.window_area.width
            || window_area.height != self.window_area.height
        {
            self.window_area = window_area;
            if self.states.is_none() {
                self.states = Some(GameStates::new(&self.args, &self.window_area))
            }
        }

        frame.render_widget(self, window_area);
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
                event::Event::Key(key) => {
                    if tx.send(Event::Input(key)).is_err() {
                        break;
                    }
                }
                event::Event::Resize(c, r) => {
                    if tx.send(Event::Resize(c, r)).is_err() {
                        break;
                    }
                }
                _ => {}
            },
            Err(_) => continue,
        }
    }
}
