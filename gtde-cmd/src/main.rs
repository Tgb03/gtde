pub mod args;
pub mod commands;
pub mod config;
pub mod error;
pub mod file_utils;
pub mod loadable;
pub mod manifest;

use args::Cli;
use clap::Parser;

fn main() {
    #[allow(unused)]
    let cli = Cli::parse();
}
