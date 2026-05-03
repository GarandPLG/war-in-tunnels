use crate::{
    app::{
        states::{
            PerkDecks,
            skirmish_states::{GameMode, ZoomLevel},
        },
        threads::Soundtrack,
        view::View,
    },
    logs::init_logger,
};
use clap::{Error, Parser, error::ErrorKind, value_parser};
use std::num::ParseFloatError;

/// Holds all configurable options that can be passed via the command line.
///
/// Each field corresponds to a CLI flag (e.g. `--view`, `--username`, …).
/// The `clap` attributes describe the flag name, help text, default value,
/// and any validation constraints.  The struct derives `Parser` so that
/// `Cli::parse()` can be called directly to obtain a populated instance.
#[derive(Parser, Debug, Clone)]
#[command(version, about = "War in Tunnels", long_about = "War in Tunnels")]
pub struct Cli {
    /// The initial view/window to display.
    #[arg(
        long,
        help = "Default window",
        value_name = "...",
        default_value_t = View::MainMenu,
        value_enum
    )]
    pub view: View,

    /// Player name shown in the UI.
    #[arg(
        long,
        help = "Username",
        value_name = "String",
        default_value = "Player"
    )]
    pub username: String,

    /// Game mode selection.
    #[arg(
        long,
        help = "Game mode",
        value_name = "...",
        default_value_t = GameMode::LastManStanding,
        value_enum
    )]
    pub game_mode: GameMode,

    /// Width of the generated map (36–100 tiles).
    #[arg(
        long,
        help = "Map width",
        value_name = "Positive integer [36; 108]",
        default_value = "42",
        value_parser = value_parser!(u8).range(36..=108)
    )]
    pub map_width: u8,

    /// Height of the generated map (11–50 tiles).
    #[arg(
        long,
        help = "Map height",
        value_name = "Positive integer [11; 39]",
        default_value = "11",
        value_parser = value_parser!(u8).range(11..=39)
    )]
    pub map_height: u8,

    /// Zoom level used for the UI.
    #[arg(
        long,
        help = "Zoom level",
        value_name = "...",
        default_value_t = ZoomLevel::Default,
        value_enum
    )]
    pub zoom_level: ZoomLevel,

    /// Which perk deck to use.
    #[arg(
        long,
        help = "Perk Deck",
        value_name = "...",
        default_value_t = PerkDecks::Silesian,
        value_enum
    )]
    pub perk_deck: PerkDecks,

    /// Starting amount of wood (1–150).
    #[arg(
        long,
        help = "Starting wood",
        value_name = "Positive integer [1; 150]",
        default_value = "50",
        value_parser = value_parser!(u16).range(1..=150)
    )]
    pub starting_wood: u16,

    /// Starting amount of iron (1–150).
    #[arg(
        long,
        help = "Starting iron",
        value_name = "Positive integer [1; 150]",
        default_value = "25",
        value_parser = value_parser!(u16).range(1..=150)
    )]
    pub starting_iron: u16,

    /// Maximum amount of units (49–149).
    #[arg(
        long,
        help = "Supply limit",
        value_name = "Positive integer [49; 149]",
        default_value = "99",
        value_parser = value_parser!(u8).range(49..=149)
    )]
    pub supply_limit: u8,

    /// Modifier applied to experience points (0.5–2.0).
    #[arg(
        long,
        help = "XP modifier",
        value_name = "Float [0.5; 2.0]",
        default_value = "1.0",
        value_parser = parse_xp
    )]
    pub xp_modifier: f32,

    /// Upper bound for skill points that can be earned (90–690).
    #[arg(
        long,
        help = "Skill points limit",
        value_name = "Positive integer [90; 690]",
        default_value = "120",
        value_parser = value_parser!(u16).range(90..=690)
    )]
    pub skill_points_limit: u16,

    /// Which soundtrack to play (default: default)
    #[arg(
            long,
            help = "Soundtrack",
            value_name = "...",
            default_value_t = Soundtrack::Default,
            value_enum
        )]
    pub sound_track: Soundtrack,

    /// Mute soundtrack (default: disabled).
    #[arg(
        long,
        help = "Mute soundtrack (default: disabled)",
        default_value_t = false
    )]
    pub mute: bool,

    /// Enable logging to a file (default: disabled).
    #[arg(
        long,
        help = "Enable logging to file (default: disabled)",
        default_value_t = false
    )]
    pub log: bool,
}

/// Parses the command line arguments and returns a fully populated `Cli`
/// instance.  This function simply forwards to `Cli::parse()`, which
/// handles argument validation and displays helpful error messages if
/// the user supplies invalid input.
pub fn get_args() -> Cli {
    let args: Cli = Cli::parse();

    if args.log {
        init_logger();
    }

    args
}

/// Parses a string into a floating‑point XP modifier and validates that it
/// lies within the inclusive range `0.5..=2.0`.
///
/// The function is used as a custom `value_parser` for the `--xp-modifier`
/// flag.  On failure it returns a `clap::Error` with `ErrorKind::InvalidValue`,
/// allowing `clap` to present a clear error message to the user.
///
/// # Errors
///
/// * Returns an error if the string cannot be parsed as `f32`.
/// * Returns an error if the parsed value is outside the allowed range.
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
