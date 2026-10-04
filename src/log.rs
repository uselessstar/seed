mod format;
mod style;
mod time;

use core::cell::RefCell;
use core::str::FromStr;
use core::sync::atomic::{AtomicBool, Ordering};
use std::io::{self, IsTerminal, Write};

use log::{LevelFilter, Log, Metadata, Record, SetLoggerError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Error while initializing the logger: {0}")]
    InitError(#[from] SetLoggerError),
}

type Result<T> = core::result::Result<T, LoggerError>;

/// Default level. Debug: all levels. Release: info, warn and error only.
/// Can be overridden at runtime with `SEED_LOG` (e.g. `SEED_LOG=debug`).
const DEFAULT_LEVEL: LevelFilter = if cfg!(debug_assertions) {
    LevelFilter::Trace
} else {
    LevelFilter::Info
};

/// Decided once at init, read with a relaxed load per record.
static COLOR: AtomicBool = AtomicBool::new(false);

struct SeedLogger;

static LOGGER: SeedLogger = SeedLogger;

thread_local! {
    /// Reused per thread: no allocation after the first few records.
    static BUF: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Initializes the logger for SEED.
///
/// Colors are enabled only when stderr is a terminal and `NO_COLOR` is unset.
///
/// # Errors
/// - Returns [`LoggerError::InitError`] if the logger fails to initialize.
pub fn init_logger() -> Result<()> {
    let color = io::stderr().is_terminal() && std::env::var_os("NO_COLOR").is_none();
    COLOR.store(color, Ordering::Relaxed);

    let level = std::env::var("SEED_LOG")
        .ok()
        .and_then(|v| LevelFilter::from_str(&v).ok())
        .unwrap_or(DEFAULT_LEVEL);

    log::set_logger(&LOGGER)?;
    log::set_max_level(level);
    Ok(())
}

impl Log for SeedLogger {
    // stderr is unbuffered and every record is written with a single
    // `write_all`, so there is nothing to flush.
    fn flush(&self) {}

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    // The `log` macros already compare against `max_level()` before calling
    // this, so the level is not checked again here.
    fn log(&self, record: &Record) {
        let color = COLOR.load(Ordering::Relaxed);

        let emit = |buf: &mut Vec<u8>| {
            buf.clear();
            format::format_record(buf, record, color);
            let _ = io::stderr().lock().write_all(buf);
        };

        BUF.with(|cell| match cell.try_borrow_mut() {
            Ok(mut buf) => {
                emit(&mut buf);
                // Do not keep a huge buffer alive after one giant message.
                if buf.capacity() > 64 * 1024 {
                    *buf = Vec::new();
                }
            }
            // A `Display` impl logged while we were formatting: use a scratch buffer.
            Err(_) => emit(&mut Vec::new()),
        });
    }
}
