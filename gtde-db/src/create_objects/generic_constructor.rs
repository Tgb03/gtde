use std::path::Path;

use gtde_error::error::Error;

pub trait GenericConstructor {
    fn construct_all(self, env_path: impl AsRef<Path>) -> Result<(), Error>;
}
