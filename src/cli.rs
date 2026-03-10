use clap::Parser;

use crate::app::View;

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
    pub window: View,

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
        value_name = "LastManStanding or FrontLines",
        default_value = "LastManStanding"
    )]
    pub game_mode: String,

    #[arg(
        long,
        help = "Map width",
        value_name = "Positive integer [20; 100]",
        default_value = "50"
    )]
    pub map_width: u8,

    #[arg(
        long,
        help = "Map height",
        value_name = "Positive integer [11; 50]",
        default_value = "25"
    )]
    pub map_height: u8,

    #[arg(
        long,
        help = "Perk Deck",
        value_name = "String (check Perk Deck tab)",
        default_value = "Silesian"
    )]
    pub perk_deck: String,

    #[arg(
        long,
        help = "Starting wood",
        value_name = "Positive integer",
        default_value = "50"
    )]
    pub starting_wood: u8,

    #[arg(
        long,
        help = "Starting iron",
        value_name = "Positive integer",
        default_value = "25"
    )]
    pub starting_iron: u8,

    #[arg(
        long,
        help = "Supply limit",
        value_name = "Positive integer",
        default_value = "99"
    )]
    pub supply_limit: u8,

    #[arg(
        long,
        help = "XP modifier",
        value_name = "Float [0.5; 2.0]",
        default_value = "1.0"
    )]
    pub xp_modifier: f32,

    #[arg(
        long,
        help = "Skill points limit",
        value_name = "Positive integer [120; 690]",
        default_value = "120"
    )]
    pub skill_points_limit: u16,
}

pub fn get_args() -> Cli {
    Cli::parse()
}
