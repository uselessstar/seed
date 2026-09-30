#![doc = include_str!("../README.md")]

use anyhow::Result;
use log::info;
use seed::{init_logger, print_header};

fn main() -> Result<()> {
    init_logger()?;
    print_header();
    info!("Initialized Logger.");
    Ok(())
}
