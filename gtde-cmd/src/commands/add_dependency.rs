use std::path::Path;
use gtde_error::error::Error;

use crate::{loadable::Loadable, manifest::Manifest};

pub fn add_dependency(env_path: &Path, dependency_path: String) -> Result<(), Error> {
    let mut config = Manifest::load(env_path)?;
    config.dependencies.insert(dependency_path.into());
    config.save(env_path)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{commands::add_dependency::add_dependency, loadable::Loadable, manifest::Manifest};
    use tempfile::{TempDir, tempdir};

    fn setup() -> TempDir {
        let dir = tempdir().unwrap();
        let mut manifest = Manifest::default();
        manifest.dependencies.insert("Dependency1".into());
        manifest.dependencies.insert("Dependency2".into());
        let _ = manifest.save(dir.path());

        dir
    }

    #[test]
    fn test_add_dependency_new() {
        let dir = setup();
        assert!(dir.path().join("manifest.json").exists());
        add_dependency(dir.path(), "Dependency3".into()).unwrap();
        let manifest = Manifest::load(dir.path()).unwrap();
        assert_eq!(manifest.dependencies.len(), 3);
    }

    #[test]
    fn test_add_dependency_already_existing() {
        let dir = setup();
        assert!(dir.path().join("manifest.json").exists());
        add_dependency(dir.path(), "Dependency1".into()).unwrap();
        let manifest = Manifest::load(dir.path()).unwrap();
        assert_eq!(manifest.dependencies.len(), 2);
    }
}
