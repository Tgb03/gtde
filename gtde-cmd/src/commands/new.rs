use std::{fs, path::Path};

use crate::commands::init;

pub fn new<'a>(name: String, path: &'a Path) -> std::io::Result<()> {
    fs::create_dir_all(path.join(&name))?;
    init::init(path)?;

    Ok(())
}
