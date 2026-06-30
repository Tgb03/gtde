use clap::{Parser, ValueEnum};
use std::{env, fmt::Display};

use crate::commands::{add_dependency, build, init, new, set_profile_path};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum VersionType {
    Debug,
    Release,
}

impl Display for VersionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Parser, Debug)]
#[command(name = "gtde")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    New {
        project_name: String,
    },
    Init,
    Build {
        #[arg(short = 'r', long = "release")]
        release: bool,
    },
    AddDependency {
        path: String,
    },
    SetProfilePath {
        path: String,
    },
}

impl Command {
    #[allow(unused)]
    pub fn solve_command(self) {
        let Ok(env_path) = env::current_dir() else {
            return;
        };

        match self {
            Command::Build { release } => {
                let version = match release {
                    true => VersionType::Release,
                    false => VersionType::Debug,
                };

                let _ = build::build(version, &env_path);
            }
            Command::New { project_name } => {
                let _ = new::new(project_name, &env_path);
            }
            Command::Init => {
                let _ = init::init(&env_path);
            }
            Command::AddDependency { path } => {
                let _ = add_dependency::add_dependency(&env_path, path);
            }
            Command::SetProfilePath { path } => {
                let _ = set_profile_path::set_profile_path(&env_path, path);
            }
        }
    }
}
