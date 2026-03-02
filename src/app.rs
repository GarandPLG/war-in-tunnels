use crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::Rect, widgets::Widget};
use std::{
    io::Result,
    sync::mpsc::{self, Receiver},
};

pub struct App {
    pub exit: bool,
}

pub enum Event {
    Input(KeyEvent),
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal, rx: Receiver<Event>) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame: &mut Frame<'_>| self.draw(frame))?;

            let event: Event = match rx.recv() {
                Ok(ev) => ev,
                Err(_) => break,
            };
            match event {
                Event::Input(key_event) => self.handle_key_event(key_event)?,
            }
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame<'_>) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
        if key_event.kind == KeyEventKind::Press && key_event.code == KeyCode::Char('q') {
            self.exit = true;
        }

        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
    }
}

pub fn handle_input_events(tx: mpsc::Sender<Event>) {
    loop {
        match event::read() {
            Ok(ev) => {
                if let event::Event::Key(key_event) = ev {
                    if tx.send(Event::Input(key_event)).is_err() {
                        break;
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}
