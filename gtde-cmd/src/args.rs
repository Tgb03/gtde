use clap::{Parser, ValueEnum};
use std::{env, fmt::Display};

use crate::commands::{add_dependency, build, create::{self, CreateFiles}, init, new, set_profile_path};
use gtde_error::error::Error;

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

impl Cli {
    pub fn run(self) -> Result<(), Error> {
        self.command.solve_command()
    }
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
    Create {
        file_used: CreateFiles,
    },
}

impl Command {
    pub fn solve_command(self) -> Result<(), Error> {
        let env_path = env::current_dir().expect("Invalid current directory.");

        match self {
            Command::Build { release } => {
                let version = match release {
                    true => VersionType::Release,
                    false => VersionType::Debug,
                };

                build::build(version, &env_path)
            }
            Command::New { project_name } => {
                new::new(project_name, &env_path).map_err(|e| e.into())
            }
            Command::Init => init::init(&env_path).map_err(|e| e.into()),
            Command::AddDependency { path } => add_dependency::add_dependency(&env_path, path),
            Command::SetProfilePath { path } => set_profile_path::set_profile_path(&env_path, path),
            Command::Create { file_used } => create::create(&env_path, file_used),
        }
    }
}
