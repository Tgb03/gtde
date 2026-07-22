use clap::{Parser, ValueEnum};
use std::{env, fmt::Display};

use crate::commands::{
    add_dependency, build,
    create::{self, CreateFiles},
    grab_db,
    init::{self, DatablockEnum},
    new,
    search_db::search_db,
    set_profile_path,
};
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
    /// Create a new gtde project in a new directory
    New {
        /// Name of the project to create
        project_name: String,
    },
    /// Initialize gtde in the current directory
    Init,
    /// Build the current project
    Build {
        /// Build release version too, debug is built no matter what.
        #[arg(short = 'r', long = "release")]
        release: bool,
    },
    /// Add a dependency to the current project
    AddDependency {
        /// Path to the dependency to add
        path: String,
    },
    /// Set the path used for the active profile
    SetProfilePath {
        /// Path to use as the profile path
        path: String,
    },
    /// Fetch a datablock from the project or embedded resources
    GrabDB {
        /// Which datablock to grab
        db: DatablockEnum,
    },
    /// Create a new entry in a datablock based on a create file
    Create {
        /// Which file type to create
        file_used: CreateFiles,
    },
    /// Search a datablock for an object by its persistent ID
    SearchDB {
        /// Name of the datablock to search, e.g. "Archetype"
        /// The datablock is just the tiny name, ignoring the prefix and suffix.
        db: DatablockEnum,
        /// The persistentID to search for
        id: u32,
        /// Only show a custom field in each found entry
        #[arg(short = 'f', long = "custom-field")]
        custom_field: Option<String>,
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
            Command::GrabDB { db } => grab_db::grab_db(&env_path, db),
            Command::SearchDB {
                db,
                id,
                custom_field,
            } => search_db(&env_path, db, id, custom_field),
        }
    }
}
