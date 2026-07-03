use std::{fs, path::Path};

use crate::error::Error;

pub fn copy_folder_by_name<'a>(
    source: impl AsRef<Path>,
    destination: impl AsRef<Path>,
    name: &'a str,
    recursive: bool,
) -> Result<(), Error> {
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
) -> Result<FileStatus, Error> {
    let path = folder.as_ref().join(file_name);

    if path.exists() {
        return Ok(FileStatus::FileExisted);
    }

    let _ = fs::create_dir_all(&folder)
        .map_err(Error::io_at(&folder))?;
    let _ = fs::write(&path, default_data)
        .map_err(Error::io_at(&folder))?;

    Ok(FileStatus::FileCreated)
}

pub fn copy_folder<'a>(source: &'a Path, destination: &'a Path, recursive: bool) -> Result<(), Error> {

    if source.is_dir() {
        for entry in fs::read_dir(source).map_err(Error::io_at(source))? {
            let entry = entry.map_err(Error::io_at(source))?;
            let src = entry.path();
            let dst = destination.join(entry.file_name());
            fs::create_dir_all(&destination).map_err(Error::io_at(&destination))?;
    
            if src.is_dir() {
                if recursive {
                    copy_folder(&src, &dst, true)?;
                }
            } else {
                fs::copy(&src, &dst).map_err(Error::io_at(&src))?;
            }
        }
    } else {
        fs::create_dir_all(destination.parent().unwrap()).map_err(Error::io_at(&destination))?;
        fs::copy(&source, &destination).map_err(Error::io_at(&source))?;
    }

    Ok(())
}
