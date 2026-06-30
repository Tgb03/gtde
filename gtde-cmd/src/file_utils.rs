use std::{fs, io::Result, path::Path};

pub fn copy_folder_by_name<'a>(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    name: &'a str,
    recursive: bool,
) -> Result<()> {
    copy_folder(
        &source.as_ref().join(name),
        &destination.as_ref().join(name),
        recursive,
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    FileExisted,
    FileCreated,
}

pub fn create_file_if_doesnt_exist(
    folder: impl AsRef<Path>,
    file_name: &str,
    default_data: impl AsRef<[u8]>,
) -> Result<FileStatus> {
    let path = folder.as_ref().join(file_name);

    if path.exists() {
        return Ok(FileStatus::FileExisted);
    }

    let _ = fs::create_dir_all(&folder)?;
    let _ = fs::write(&path, default_data)?;

    Ok(FileStatus::FileCreated)
}

fn copy_folder<'a>(source: &'a Path, destination: &'a Path, recursive: bool) -> Result<()> {
    fs::create_dir_all(source)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let src = entry.path();
        let dst = destination.join(entry.file_name());

        if src.is_dir() {
            if recursive {
                copy_folder(&src, &dst, true)?;
            }
        } else {
            fs::copy(&src, &dst)?;
        }
    }

    Ok(())
}
