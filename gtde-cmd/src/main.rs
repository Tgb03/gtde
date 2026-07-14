pub mod args;
pub mod commands;
pub mod config;
pub mod file_utils;
pub mod loadable;
pub mod manifest;
pub mod named_data;

use args::Cli;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    if let Err(e) = cli.run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
