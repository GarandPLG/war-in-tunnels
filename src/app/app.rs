use crate::{
    app::{
        GameStates,
        keybindings::{Action, event_to_action, main_menu_keybindings},
    },
    cli::Cli,
};
use clap::ValueEnum;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, KeyEvent},
};
use std::{
    io::Result,
    sync::mpsc::{self, Receiver},
};

pub enum Event {
    Input(KeyEvent),
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
pub enum View {
    MainMenu,
    Skirmish,
    PerkDecks,
    SkillsConfig,
    Settings,
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
        match self.view {
            View::MainMenu => main_menu_keybindings(self, &key_event),
            _ => {
                if let Some(action) = event_to_action(&key_event) {
                    match action {
                        Action::Quit => self.exit = true,
                        Action::Esc => self.view = View::MainMenu,
                        _ => (),
                    }
                }
            }
        }

        Ok(())
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
