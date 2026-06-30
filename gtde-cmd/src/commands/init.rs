use std::{fs, io::Result, path::Path};

use crate::{config::Config, file_utils};

pub fn init<'a>(path: &'a Path) -> Result<()> {
    fs::create_dir_all(path.join("Assets"))?;
    fs::create_dir_all(path.join("config"))?;
    fs::create_dir_all(path.join("plugins"))?;
    fs::create_dir_all(path.join("Custom"))?;
    file_utils::create_file_if_doesnt_exist(path, "CHANGELOG.md", "")?;
    file_utils::create_file_if_doesnt_exist(path, "README.md", "")?;
    file_utils::create_file_if_doesnt_exist(path, "manifest.json", "")?;

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
