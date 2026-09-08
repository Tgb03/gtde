/// File AI generated with ChatGPT

use std::fs;
use std::io;
use std::path::Path;

/// Ensures that the given entries exist in `.gitignore`.
///
/// Missing entries are appended to the file. Existing entries are left unchanged.
pub fn update_gitignore<P, I, S>(env_path: P, entries: I) -> io::Result<()>
where
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let path = env_path.as_ref()
        .join(".gitignore")
        .with_extension("");

    // Load the existing .gitignore, or treat a missing file as empty.
    let mut contents = match fs::read_to_string(&path) {
        Ok(contents) => contents,
        Err(err) if err.kind() == io::ErrorKind::NotFound => String::new(),
        Err(err) => return Err(err),
    };

    let mut added = Vec::new();

    for entry in entries {
        let entry = entry.as_ref();

        // Check complete lines rather than using `contains`, so that
        // e.g. "target" doesn't incorrectly match "target/debug".
        let exists = contents.lines().any(|line| line.trim() == entry);

        if !exists {
            added.push(entry.to_string());
        }
    }

    if added.is_empty() {
        return Ok(());
    }

    // Make sure the existing contents end with a newline before appending.
    if !contents.is_empty() && !contents.ends_with('\n') {
        contents.push('\n');
    }

    for entry in added {
        contents.push_str(&entry);
        contents.push('\n');
    }

    fs::write(path, contents)
}
