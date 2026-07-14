use std::{fs, path::Path};

use crate::commands::init;
use gtde_error::error::Error;

pub fn new<'a>(name: String, path: &'a Path) -> Result<(), Error> {
    let path = path.join(&name);
    fs::create_dir_all(&path).map_err(Error::io_at(&path))?;

    init::init(&path)?;

    Ok(())
}
