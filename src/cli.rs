use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "War in Tunnels", long_about = "War in Tunnels")]
pub struct Cli {}

pub fn get_args() -> Cli {
    Cli::parse()
}
