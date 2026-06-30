use std::{fs, path::Path};

use crate::{commands::init, error::Error};

pub fn new<'a>(name: String, path: &'a Path) -> Result<(), Error> {
    fs::create_dir_all(path.join(&name))
        .map_err(Error::io_at(path.join(&name)))?;
    
    init::init(path)?;

    Ok(())
}
