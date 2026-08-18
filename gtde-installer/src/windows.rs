
const ENVIRONMENT: &str = "Environment";
const PATH: &str = "Path";
const DELIMITER: char = ';';

use windows_registry::Key;

/// Appends `path` to the user's `PATH` environment variable.
pub fn add_to_path(path: &str) -> std::io::Result<()> {
    eprintln!("\x1b[38;5;248madding {path:?} to PATH...\x1b[0m");

    println!("opening the environment registry key...");
    let key = open_user_environment_key()?;

    println!("getting PATH environment variable...");
    let mut path_var = key.get_string(PATH).map_err(std::io::Error::other)?;
    if path_var
        .rsplit(DELIMITER) // using `rsplit` because it'll likely be near the end
        .any(|p| p == path)
    {
        eprintln!("\x1b[38;5;248malready in PATH. operation aborted\x1b[0m");
        return Ok(());
    } else {
        println!("{path:?} not in PATH. adding...");
    }

    if !path_var.ends_with(DELIMITER) {
        path_var.push(DELIMITER);
    }
    path_var.push_str(path);

    write_to_path_variable(&key, path_var)?;
    eprintln!("\x1b[38;5;248msuccessfully added {path:?} to PATH\x1b[0m");
    Ok(())
}

/// Opens the user's environment registry key in read/write mode.
fn open_user_environment_key() -> std::io::Result<Key> {
    windows_registry::CURRENT_USER
        .options()
        .read()
        .write()
        .open(ENVIRONMENT)
        .map_err(std::io::Error::other)
}

/// Write `value` to the user's `PATH` environment variable.
fn write_to_path_variable(key: &Key, value: String) -> std::io::Result<()> {
    key.set_string(PATH, value).map_err(std::io::Error::other)
}