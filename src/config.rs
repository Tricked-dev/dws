use clap::Parser;
use once_cell::sync::Lazy;

use crate::cli::Cli;

pub static CONFIG: Lazy<Cli> = Lazy::new(Cli::parse);
