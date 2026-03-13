use crate::{
    app::{GameStates, handle_keybindings, view::View},
    cli::Cli,
};
use clap::ValueEnum;
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
    pub game_states: GameStates,
    pub username: String,
    pub game_mode: GameMode,
    pub map_width: u8,
    pub map_height: u8,
    pub perk_deck: PerkDecks,
    pub starting_wood: u8,
    pub starting_iron: u8,
    pub supply_limit: u8,
    pub xp_modifier: f32,
    pub skill_points_limit: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum GameMode {
    LastManStanding,
    Frontlines,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, ValueEnum)]
pub enum PerkDecks {
    Silesian,
    BogeyMan,
    Anteater,
}

impl App {
    pub fn new(args: Cli) -> Self {
        Self {
            exit: false,
            game_states: GameStates::new(),
            view: args.view,
            username: args.username,
            game_mode: args.game_mode,
            map_width: args.map_width,
            map_height: args.map_height,
            perk_deck: args.perk_deck,
            starting_wood: args.starting_wood,
            starting_iron: args.starting_iron,
            supply_limit: args.supply_limit,
            xp_modifier: args.xp_modifier,
            skill_points_limit: args.skill_points_limit,
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

    fn draw(&self, frame: &mut Frame<'_>) {
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
