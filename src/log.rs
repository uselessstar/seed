use core::fmt::{self, Write as _};
use std::io::{self, Write};

mod time;

use chrono::{Datelike, Local, Timelike};
use log::{Level, LevelFilter, Log, Metadata, Record, SetLoggerError, set_logger};
use owo_colors::{OwoColorize, Style};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("Error while initializing the logger: {0}")]
    InitError(#[from] SetLoggerError),
}

type Result<T> = core::result::Result<T, LoggerError>;

const MAX_LEVEL: LevelFilter = if cfg!(debug_assertions) {
    LevelFilter::Trace
} else {
    LevelFilter::Info
};

struct StackBuf<'a, const N: usize> {
    buf: [u8; N],
    len: usize,
    inner: &'a mut dyn Write,
}

impl<'a, const N: usize> StackBuf<'a, N> {
    fn new(inner: &'a mut dyn Write) -> Self {
        Self {
            buf: [0; N],
            len: 0,
            inner,
        }
    }

    fn drain(&mut self) -> io::Result<()> {
        let res = self.inner.write_all(&self.buf[..self.len]);
        self.len = 0;
        res
    }
}

impl<const N: usize> Write for StackBuf<'_, N> {
    fn write(&mut self, data: &[u8]) -> io::Result<usize> {
        if data.len() > N - self.len {
            self.drain()?;
            if data.len() >= N {
                return self.inner.write(data);
            }
        }
        self.buf[self.len..self.len + data.len()].copy_from_slice(data);
        self.len += data.len();
        Ok(data.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.drain()?;
        self.inner.flush()
    }
}

struct SeedLogger;

static LOGGER: SeedLogger = SeedLogger;

/// Initializes the logger for SEED.
/// Debug: all levels. Release: info, warn and error only.
///
/// # Errors
/// - Returns [`LoggerError::InitError`] if the logger fails to initialize.
pub fn init_logger() -> Result<()> {
    set_logger(&LOGGER)?;
    log::set_max_level(MAX_LEVEL);
    Ok(())
}

/// Uppercases the target while formatting, without allocating a `String`.
struct Upper<'a>(&'a str);

impl fmt::Display for Upper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .chars()
            .try_for_each(|c| f.write_char(c.to_ascii_uppercase()))
    }
}

/// (level style, target style)
fn level_style(level: Level) -> Style {
    match level {
        Level::Trace | Level::Debug => Style::new().dimmed(),
        Level::Info => Style::new().cyan(),
        Level::Warn => Style::new().yellow(),
        Level::Error => Style::new().red(),
    }
}

fn write_record(out: &mut dyn Write, record: &Record) -> io::Result<()> {
    let mut out = StackBuf::<512>::new(out);
    let level = record.level();
    let level_style = level_style(level);
    let t = Local::now().naive_local();

    write!(
        out,
        "[{:04}/{:02}/{:02} {:02}:{:02}:{:02}][{}][{}]",
        t.year(),
        t.month(),
        t.day(),
        t.hour(),
        t.minute(),
        t.second(),
        level.as_str().style(level_style),
        Upper(record.target()).bright_white(),
    )?;

    if cfg!(debug_assertions) {
        write!(
            out,
            " {}",
            format_args!(
                "{}:{}",
                record.file_static().unwrap_or("unknown"),
                record.line().unwrap_or(0),
            )
            .dimmed(),
        )?;
    }

    writeln!(out, ": {}", record.args())?;
    out.drain()
}

impl Log for SeedLogger {
    fn flush(&self) {}

    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= MAX_LEVEL
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        let _ = if record.level() <= Level::Warn {
            write_record(&mut io::stderr().lock(), record)
        } else {
            write_record(&mut io::stdout().lock(), record)
        };
    }
}
