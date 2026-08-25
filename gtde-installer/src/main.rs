use std::{env, fs, path::PathBuf};

#[cfg(target_os = "windows")]
use crate::windows::add_to_path;
#[cfg(target_os = "windows")]
pub mod windows;

const APP_BINARY: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../target/release/gtde-cmd.exe"
));

fn install_directory() -> PathBuf {
    env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .expect("LOCALAPPDATA is not set")
        .join("Tgb03")
        .join("GTDE")
}

fn main() -> std::io::Result<()> {
    let install_dir = install_directory();

    fs::create_dir_all(&install_dir)?;
    let executable = install_dir.join("gtde.exe");
    fs::write(&executable, APP_BINARY)?;

    #[cfg(target_os = "windows")]
    add_to_path(install_directory().as_os_str().to_str().unwrap())?;

    println!("GTDE installed to: ");
    println!("{}", executable.display());

    Ok(())
}
