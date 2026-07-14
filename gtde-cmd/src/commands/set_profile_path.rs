use std::path::Path;

use gtde_error::error::Error;
use crate::{config::Config, loadable::Loadable};

pub fn set_profile_path(env_path: impl AsRef<Path>, path: impl AsRef<Path>) -> Result<(), Error> {
    let mut config = Config::load(env_path.as_ref())?;
    config.profile_path = path.as_ref().to_owned();
    config.save(env_path.as_ref())?;

    Ok(())
}
