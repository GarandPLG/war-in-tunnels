use crate::app::{
    states::{GameMode, PerkDecks, ZoomLevel},
    view::View,
};
use clap::{Error, Parser, error::ErrorKind, value_parser};
use std::num::ParseFloatError;

#[derive(Parser, Debug)]
#[command(version, about = "War in Tunnels", long_about = "War in Tunnels")]
pub struct Cli {
    #[arg(
        long,
        help = "Default window",
        value_name = "...",
        default_value_t = View::MainMenu,
        value_enum
    )]
    pub view: View,

    #[arg(
        long,
        help = "Username",
        value_name = "String",
        default_value = "Player"
    )]
    pub username: String,

    #[arg(
        long,
        help = "Game mode",
        value_name = "...",
        default_value_t = GameMode::LastManStanding,
        value_enum
    )]
    pub game_mode: GameMode,

    #[arg(
        long,
        help = "Map width",
        value_name = "Positive integer [20; 100]",
        default_value = "50",
        value_parser = value_parser!(u8).range(20..=100)
    )]
    pub map_width: u8,

    #[arg(
        long,
        help = "Map height",
        value_name = "Positive integer [11; 50]",
        default_value = "25",
        value_parser = value_parser!(u8).range(25..=50)
    )]
    pub map_height: u8,

    #[arg(
        long,
        help = "Zoom level",
        value_name = "...",
        default_value_t = ZoomLevel::Default,
        value_enum
    )]
    pub zoom_level: ZoomLevel,

    #[arg(
        long,
        help = "Perk Deck",
        value_name = "...",
        default_value_t = PerkDecks::Silesian,
        value_enum
    )]
    pub perk_deck: PerkDecks,

    #[arg(
        long,
        help = "Starting wood",
        value_name = "Positive integer [1; 150]",
        default_value = "50",
        value_parser = value_parser!(u16).range(1..=150)
    )]
    pub starting_wood: u16,

    #[arg(
        long,
        help = "Starting iron",
        value_name = "Positive integer [1; 150]",
        default_value = "25",
        value_parser = value_parser!(u16).range(1..=150)
    )]
    pub starting_iron: u16,

    #[arg(
        long,
        help = "Supply limit",
        value_name = "Positive integer [49; 149]",
        default_value = "99",
        value_parser = value_parser!(u8).range(49..=149)
    )]
    pub supply_limit: u8,

    #[arg(
        long,
        help = "XP modifier",
        value_name = "Float [0.5; 2.0]",
        default_value = "1.0",
        value_parser = parse_xp
    )]
    pub xp_modifier: f32,

    #[arg(
        long,
        help = "Skill points limit",
        value_name = "Positive integer [120; 690]",
        default_value = "120",
        value_parser = value_parser!(u16).range(120..=690)
    )]
    pub skill_points_limit: u16,

    #[arg(
        long,
        help = "Enable logging to file (default: disabled)",
        default_value_t = false
    )]
    pub log: bool,
}

pub fn get_args() -> Cli {
    Cli::parse()
}

fn parse_xp(s: &str) -> Result<f32, Error> {
    let v: f32 = s
        .parse()
        .map_err(|e: ParseFloatError| Error::raw(ErrorKind::InvalidValue, e.to_string()))?;
    if (0.5..=2.0).contains(&v) {
        Ok(v)
    } else {
        Err(Error::raw(
            ErrorKind::InvalidValue,
            format!("Value {} is out of range 0.5..=2.0", v),
        ))
    }
}
