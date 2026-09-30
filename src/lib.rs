#![doc = include_str!("../README.md")]

mod header;
mod logger;

pub use logger::init_logger;

pub use header::print_header;
