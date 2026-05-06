use crate::{
    app::{GameStates, handle_keybindings, threads::AudioCmd, view::View},
    cli::Cli,
};
use ratatui::{DefaultTerminal, Frame, crossterm::event::KeyEvent, layout::Rect};
use std::{
    io::Result,
    sync::mpsc::{Receiver, Sender},
    thread::sleep,
    time::Duration,
};

pub struct App<'a> {
    pub exit: bool,
    pub view: View,
    pub window_area: Rect,
    pub args: Cli,
    pub states: Option<GameStates<'a>>,
    pub audio_tx: Sender<AudioCmd>,
}

impl<'a> App<'a> {
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

    pub fn states_ref(&self) -> Option<&GameStates<'a>> {
        self.states.as_ref()
    }

    pub fn states_mut(&mut self) -> Option<&mut GameStates<'a>> {
        self.states.as_mut()
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        input_rx: Receiver<KeyEvent>,
        resize_rx: Receiver<(u16, u16)>,
        tick_rx: Receiver<()>,
    ) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw(frame))?;

            if let Ok(key) = input_rx.try_recv() {
                self.handle_key_event(key)?;
            }

            if let Ok((_, _)) = resize_rx.try_recv() {
                let window_area: Rect = self.window_area;
                if let Some(state) = self.states_mut() {
                    state
                        .skirmish
                        .board
                        .change_resize(&window_area, state.skirmish.side_panel);
                }
            }

            if let Ok(()) = tick_rx.try_recv() {
                self.update()?;
            }

            sleep(Duration::from_millis(10));
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

    fn update(&mut self) -> Result<()> {
        if let Some(state) = self.states_mut() {
            state.skirmish.tick_update();
        }

        Ok(())
    }
}
