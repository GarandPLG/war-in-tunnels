use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

pub fn init_logger() {
    WriteLogger::init(
        LevelFilter::Info,
        Config::default(),
        File::create("war_in_tunnels.log").expect("Failed to create log file"),
    )
    .expect("Failed to initialize logger");
}
