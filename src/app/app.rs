use crate::{
    app::{
        GameStates, handle_keybindings,
        threads::{AppEvent, AudioCmd},
        view::View,
    },
    cli::Cli,
};
use ratatui::{DefaultTerminal, Frame, crossterm::event::KeyEvent, layout::Rect};
use std::{
    io::Result,
    sync::mpsc::{Receiver, RecvTimeoutError, Sender},
    time::Duration,
};

pub struct App {
    pub exit: bool,
    pub view: View,
    pub window_area: Rect,
    pub args: Cli,
    pub states: Option<GameStates>,
    pub audio_tx: Sender<AudioCmd>,
}

impl App {
    pub fn new(args: Cli, audio_tx: Sender<AudioCmd>) -> Self {
        Self {
            exit: false,
            view: args.view,
            window_area: Rect::default(),
            args: args,
            states: None,
            audio_tx,
        }
    }

    pub fn states(&self) -> Option<&GameStates> {
        self.states.as_ref()
    }

    pub fn states_mut(&mut self) -> Option<&mut GameStates> {
        self.states.as_mut()
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<AppEvent>) -> Result<()> {
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
                AppEvent::Input(key_event) => self.handle_key_event(key_event)?,
                AppEvent::Resize(_, _) => {}
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
