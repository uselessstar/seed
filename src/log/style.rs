use log::Level;
use owo_colors::Style;

pub(super) const DIM: Style = Style::new().dimmed();
pub(super) const TARGET: Style = Style::new().bright_white();

const INFO: Style = Style::new().cyan();
const WARN: Style = Style::new().yellow();
const ERROR: Style = Style::new().red();

/// (style, padded name). Padding is baked in so no formatting is needed.
pub(super) fn level_parts(level: Level) -> (Style, &'static str) {
    match level {
        Level::Trace => (DIM, "TRACE"),
        Level::Debug => (DIM, "DEBUG"),
        Level::Info => (INFO, "INFO "),
        Level::Warn => (WARN, "WARN "),
        Level::Error => (ERROR, "ERROR"),
    }
}
