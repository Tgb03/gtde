use colored::Colorize;
use gtde_error::error::Error;
use std::{fs, path::Path};

use crate::{
    args::VersionType, config::Config, file_utils, loadable::Loadable, manifest::Manifest,
};

pub fn build(version: VersionType, env_path: &Path) -> Result<(), Error> {
    let config = Config::load(env_path)?;
    let manifest = Manifest::load(env_path)?;

    match version {
        VersionType::Debug => build_debug(env_path, &config, &manifest)?,
        VersionType::Release => build_release(env_path, config, manifest)?,
    };

    Ok(())
}

fn build_release(env_path: &Path, config: Config, mut manifest: Manifest) -> Result<(), Error> {
    let destination = &env_path.join("output").join(&manifest.name);

    build_debug(env_path, &config, &manifest)?;

    manifest.dependencies.retain(|dependency| {
        config
            .dev_dependencies
            .iter()
            .all(|dev_dependency| !dependency.contains(dev_dependency))
    });
    fs::create_dir_all(destination).map_err(Error::io_at(destination))?;
    manifest.save(&destination)?;

    file_utils::copy_folder_by_name(env_path, destination, "Assets", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "config", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "plugins", true)?;
    file_utils::copy_folder_by_name(env_path, destination.join("plugins"), "Custom", true)?;
    file_utils::copy_folder_by_name(env_path, destination, "CHANGELOG.md", false)?;
    file_utils::copy_folder_by_name(env_path, destination, "README.md", false)?;
    file_utils::copy_folder_by_name(env_path, destination, "icon.png", false)?;

    let config = Config::load(&env_path)?;
    for dll_path in config.extra_dll_locations {
        let name = dll_path.components().last().unwrap();
        fs::copy(&dll_path, destination.join(name)).map_err(Error::io_at(dll_path))?;
    }

    file_utils::zip_folder(
        destination,
        &env_path
            .join("output")
            .join(&manifest.name)
            .with_extension("zip"),
    )
    .map_err(Error::io_at(destination))?;

    println!("{}", "Release built succesfully".green());
    Ok(())
}

fn build_debug(env_path: &Path, config: &Config, manifest: &Manifest) -> Result<(), Error> {
    let destination_bepinex = config.profile_path.join("BepInEx");
    let mod_location = destination_bepinex.join("plugins").join(&manifest.name);

    file_utils::copy_folder_by_name(env_path, &destination_bepinex, "Assets", true)?;
    file_utils::copy_folder_by_name(env_path, &destination_bepinex, "config", true)?;
    file_utils::copy_folder_by_name(env_path, &mod_location.join(&manifest.name), "Custom", true)?;
    file_utils::copy_folder(
        &env_path.join("plugins"),
        &mod_location.join(&manifest.name),
        true,
    )?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "CHANGELOG.md", false)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "README.md", false)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "manifest.json", false)?;
    file_utils::copy_folder_by_name(env_path, &mod_location, "icon.png", false)?;

    let config = Config::load(&env_path)?;
    for dll_path in config.extra_dll_locations {
        let name = dll_path.components().last().unwrap();
        fs::copy(&dll_path, mod_location.join(name)).map_err(Error::io_at(dll_path))?;
    }

    println!("{}", "Debug built succesfully".green());
    Ok(())
}

#[cfg(test)]
mod tests {}
