use std::{fs, path::Path};

use gtde_error::error::Error;
use crate::{commands::init};

pub fn new<'a>(name: String, path: &'a Path) -> Result<(), Error> {
    let path = path.join(&name);
    fs::create_dir_all(&path)
        .map_err(Error::io_at(&path))?;
    
    init::init(&path)?;

    Ok(())
}
