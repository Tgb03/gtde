use colored::Colorize;
use gtde_error::error::Error;
use include_dir::{Dir, include_dir};
use std::path::Path;

use crate::commands::init::DatablockEnum;

pub static PROJECT_DATABLOCKS: Dir = include_dir!("$CARGO_MANIFEST_DIR/../resources/datablocks");

pub fn grab_db(env_path: impl AsRef<Path>, datablock: DatablockEnum) -> Result<(), Error> {
    let name = format!("{}", datablock);
    let path = env_path.as_ref().join("plugins").join(&name);
    let file = PROJECT_DATABLOCKS
        .get_file(&name)
        .ok_or(Error::FileNotFound(name.clone()))?;
    if std::fs::exists(&path).map_err(Error::io_at(&path))? {
        println!("{}", "Datablock already exists. For failsafe reasons you need to manually delete existing one for this one to replace it.".yellow());
        return Err(Error::DatablockAlreadyExists(name));
    }
    std::fs::write(&path, file.contents()).map_err(Error::io_at(path))?;

    Ok(())
}
