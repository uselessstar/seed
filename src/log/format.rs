use core::fmt::{self, Display, Write as _};
use std::io::Write;

use log::Record;
use owo_colors::{OwoColorize, Style};

use super::style::{DIM, TARGET, level_parts};
use super::time::timestamp;

/// Uppercases the target while formatting, without allocating a `String`.
struct Upper<'a>(&'a str);

impl Display for Upper<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0
            .chars()
            .try_for_each(|c| f.write_char(c.to_ascii_uppercase()))
    }
}

/// Writes `text` with `style`, or unstyled when color is off.
/// An empty `Style` emits no escape sequences.
fn paint(buf: &mut Vec<u8>, color: bool, style: Style, text: impl Display) {
    let style = if color { style } else { Style::new() };
    let _ = write!(buf, "{}", text.style(style));
}

/// Format: `[YYYY/MM/DD HH:MM:SS][LEVEL][TARGET] file:line: message`
/// (`file:line` only in debug builds).
pub(super) fn format_record(buf: &mut Vec<u8>, record: &Record, color: bool) {
    let (style, name) = level_parts(record.level());

    buf.extend_from_slice(&timestamp());

    buf.push(b'[');
    paint(buf, color, style, name);
    buf.extend_from_slice(b"][");
    paint(buf, color, TARGET, Upper(record.target()));
    buf.push(b']');

    if cfg!(debug_assertions) {
        buf.push(b' ');
        paint(
            buf,
            color,
            DIM,
            format_args!(
                "{}:{}",
                record.file_static().unwrap_or("unknown"),
                record.line().unwrap_or(0),
            ),
        );
    }

    buf.extend_from_slice(b": ");
    match record.args().as_str() {
        Some(s) => buf.extend_from_slice(s.as_bytes()),
        None => {
            let _ = write!(buf, "{}", record.args());
        }
    }
    buf.push(b'\n');
}
