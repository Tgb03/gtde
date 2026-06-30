use std::{fs, path::Path};

use crate::{
    args::VersionType, config::Config, error::Error, file_utils, loadable::Loadable,
    manifest::Manifest,
};

pub fn build(version: VersionType, env_path: &Path) -> Result<(), Error> {
    let config = Config::load(env_path).map_err(|_| Error::NoConfig)?;
    let manifest = Manifest::load(env_path).map_err(|_| Error::NoConfig)?;

    match version {
        VersionType::Debug => build_debug(env_path, config, manifest)?,
        VersionType::Release => build_release(env_path, config, manifest)?,
    };

    Ok(())
}

fn build_release(env_path: &Path, config: Config, manifest: Manifest) -> Result<(), Error> {
    let destination = &env_path.join("output");

    build_debug(env_path, config, manifest)?;

    file_utils::copy_folder_by_name(env_path, destination, "Assets", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "config", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "plugins", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "Custom", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "CHANGELOG.md", false)?;
    file_utils::copy_folder_by_name(env_path, destination, "README.md", false)?;
    file_utils::copy_folder_by_name(env_path, destination, "manifest.json", false)?;
    file_utils::copy_folder_by_name(env_path, destination, "icon.png", false)?;

    let config = Config::load(&env_path)?;
    for dll_path in config.extra_dll_locations {
        let name = dll_path.components().last().unwrap();
        fs::copy(&dll_path, destination.join(name))?;
    }

    Ok(())
}

fn build_debug(env_path: &Path, config: Config, manifest: Manifest) -> Result<(), Error> {
    let destination_bepinex = config.profile_path.join("BepInEx");
    let mod_location = destination_bepinex
        .join("plugins")
        .join(format!("{}-{}", manifest.author_name, manifest.name));
    let mod_inner = mod_location.join(&manifest.name);

    file_utils::copy_folder_by_name(env_path, &destination_bepinex, "Assets", true)?;
    file_utils::copy_folder_by_name(env_path, &destination_bepinex, "config", true)?;
    file_utils::copy_folder_by_name(env_path, &mod_inner, "plugins", true)?;
    file_utils::copy_folder_by_name(env_path, &mod_inner, "Custom", true)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "CHANGELOG.md", false)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "README.md", false)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "manifest.json", false)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "icon.png", false)?;

    let config = Config::load(&env_path)?;
    for dll_path in config.extra_dll_locations {
        let name = dll_path.components().last().unwrap();
        fs::copy(&dll_path, mod_location.join(name))?;
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {}
