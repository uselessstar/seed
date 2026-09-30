use chrono::Local;
use log::{Level, Log, SetLoggerError, set_logger};
use owo_colors::OwoColorize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Error while initializing the logger: {0}")]
    InitError(SetLoggerError),
}

type Result<T> = core::result::Result<T, LoggerError>;

struct SeedLogger;

static LOGGER: SeedLogger = SeedLogger;

/// Initializes the logger for SEED.
/// In debug mode, it will log all messages (trace, debug, info, warn, error).
/// In release mode, it will log only info, warn, and error messages.
///
/// # Errors
/// - Returns [`LoggerError::InitError`] if the logger fails to initialize.
pub fn init_logger() -> Result<()> {
    if let Err(e) = set_logger(&LOGGER) {
        return Err(LoggerError::InitError(e));
    }
    #[cfg(debug_assertions)]
    {
        log::set_max_level(log::LevelFilter::Trace);
    }
    #[cfg(not(debug_assertions))]
    {
        log::set_max_level(log::LevelFilter::Info);
    }
    Ok(())
}

impl Log for SeedLogger {
    fn flush(&self) {}
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        #[cfg(debug_assertions)]
        {
            metadata.level() <= log::Level::Trace
        }
        #[cfg(not(debug_assertions))]
        {
            metadata.level() <= log::Level::Info
        }
    }
    fn log(&self, record: &log::Record) {
        let time = Local::now().format("%Y/%m/%d %H:%M:%S");
        let level_str = record.level().as_str().to_uppercase();
        let target_str = record.target().to_uppercase();
        let args = record.args();
        #[cfg(debug_assertions)]
        {
            let file = record.file_static().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);
            let fileline = format_args!("{}:{}", file, line);
            let msg = match record.level() {
                Level::Trace | Level::Debug => format_args!(
                    "[{}][{}][{}] {}: {}",
                    time,
                    level_str.dimmed(),
                    target_str.dimmed(),
                    fileline.dimmed(),
                    args
                ),
                Level::Info => format_args!(
                    "[{}][{}][{}] {}: {}",
                    time,
                    level_str.cyan(),
                    target_str.white(),
                    fileline.dimmed(),
                    args
                ),
                Level::Warn => format_args!(
                    "[{}][{}][{}] {}: {}",
                    time,
                    level_str.yellow(),
                    target_str.white(),
                    fileline.dimmed(),
                    args
                ),
                Level::Error => format_args!(
                    "[{}][{}][{}] {}: {}",
                    time,
                    level_str.red(),
                    target_str.white(),
                    fileline.dimmed(),
                    args
                ),
            };

            match record.level() {
                Level::Trace | Level::Debug | Level::Info => println!("{}", msg),
                Level::Warn | Level::Error => eprintln!("{}", msg),
            }
        }
        #[cfg(not(debug_assertions))]
        {
            let msg = match record.level() {
                Level::Trace | Level::Debug => format_args!(
                    "[{}][{}][{}]: {}",
                    time,
                    level_str.dimmed(),
                    target_str.dimmed(),
                    args
                ),
                Level::Info => format_args!(
                    "[{}][{}][{}]: {}",
                    time,
                    level_str.cyan(),
                    target_str.white(),
                    args
                ),
                Level::Warn => format_args!(
                    "[{}][{}][{}]: {}",
                    time,
                    level_str.yellow(),
                    target_str.white(),
                    args
                ),
                Level::Error => format_args!(
                    "[{}][{}][{}]: {}",
                    time,
                    level_str.red(),
                    target_str.white(),
                    args
                ),
            };
            match record.level() {
                Level::Info => println!("{}", msg),
                Level::Warn | Level::Error => eprintln!("{}", msg),
                _ => {}
            }
        }
    }
}
