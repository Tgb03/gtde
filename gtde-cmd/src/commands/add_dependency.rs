use gtde_error::error::Error;
use gtde_file::loadable::Loadable;
use std::path::Path;

use crate::manifest::Manifest;

pub fn add_dependency(env_path: &Path, dependency_path: String) -> Result<(), Error> {
    let mut manifest = Manifest::load(env_path, "manifest")?;
    manifest.dependencies.insert(dependency_path.into());
    manifest.save(env_path, "manifest")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{commands::add_dependency::add_dependency, manifest::Manifest};
    use gtde_file::loadable::Loadable;
    use tempfile::{TempDir, tempdir};

    fn setup() -> TempDir {
        let dir = tempdir().unwrap();
        let mut manifest = Manifest::default();
        manifest.dependencies.insert("Dependency1".into());
        manifest.dependencies.insert("Dependency2".into());
        let _ = manifest.save(dir.path(), "manifest");

        dir
    }

    #[test]
    fn test_add_dependency_new() {
        let dir = setup();
        assert!(dir.path().join("manifest.json").exists());
        add_dependency(dir.path(), "Dependency3".into()).unwrap();
        let manifest = Manifest::load(dir.path(), "manifest").unwrap();
        assert_eq!(manifest.dependencies.len(), 3);
    }

    #[test]
    fn test_add_dependency_already_existing() {
        let dir = setup();
        assert!(dir.path().join("manifest.json").exists());
        add_dependency(dir.path(), "Dependency1".into()).unwrap();
        let manifest = Manifest::load(dir.path(), "manifest").unwrap();
        assert_eq!(manifest.dependencies.len(), 2);
    }
}
