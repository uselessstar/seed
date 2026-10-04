#![doc = include_str!("../README.md")]

mod header;
mod log;

pub use log::init_logger;

pub use header::print_header;
