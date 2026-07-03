use std::{fs, path::Path};

use crate::{config::Config, error::Error, file_utils, manifest::Manifest};

pub fn init<'a>(path: &'a Path) -> Result<(), Error> {
    fs::create_dir_all(path.join("Assets"))
        .map_err(Error::io_at(path.join("Assets")))?;
    fs::create_dir_all(path.join("config"))
        .map_err(Error::io_at(path.join("config")))?;
    fs::create_dir_all(path.join("plugins"))
        .map_err(Error::io_at(path.join("plugins")))?;
    fs::create_dir_all(path.join("Custom"))
        .map_err(Error::io_at(path.join("Custom")))?;
    file_utils::create_file_if_doesnt_exist(path, "CHANGELOG.md", "")?;
    file_utils::create_file_if_doesnt_exist(path, "README.md", "")?;
    file_utils::create_file_if_doesnt_exist(path, "manifest.json", 
        serde_json::to_string_pretty(&Manifest::default()).unwrap_or_default()
    )?;

    let config_data = serde_json::to_string_pretty(&Config::default()).unwrap_or_default();
    file_utils::create_file_if_doesnt_exist(path, "gtde.config", config_data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::commands::init::init;
    use tempfile::tempdir;

    #[test]
    fn test_simple_init() {
        let temp = tempdir().unwrap();
        assert!(init(temp.path()).is_ok());
    }
}
