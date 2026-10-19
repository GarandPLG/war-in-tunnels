use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;

/// Initializes the global logger for the application.
///
/// This function creates (or truncates) a file called `war_in_tunnels.log`
/// in the current working directory and configures a `simplelog::WriteLogger`
/// to write log messages with a minimum level of `Info`.
///
/// # Panics
///
/// - If the log file cannot be created (e.g., due to permission issues).
/// - If the logger cannot be initialized (which would indicate an internal
///   error in the `simplelog` crate).
///
/// Call this once, preferably at the very beginning of `main()`, before any
/// logging macros are used.
///
/// # Example
///
/// ```
/// use war_in_tunnels::logs::init_logger;
/// use log::info;
///
/// fn main() {
///     init_logger();
///     info!("Application started");
/// }
/// ```
pub fn init_logger() {
  WriteLogger::init(
    LevelFilter::Info,
    Config::default(),
    File::create("war_in_tunnels.log").expect("Failed to create log file"),
  )
  .expect("Failed to initialize logger");
}
