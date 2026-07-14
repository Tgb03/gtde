use std::{fs::{self, File}, io::{Read, Write}, path::Path};

use gtde_error::error::Error;
use walkdir::WalkDir;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

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

pub fn zip_folder(src_dir: &Path, out_dir: &Path) -> std::io::Result<()> {
    if !src_dir.is_dir() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{} is not a directory", src_dir.display()),
        ));
    }

    // Build output path: same name as folder + ".zip"
    let zip_path = out_dir;

    let zip_file = File::create(&zip_path)?;
    let mut zip = ZipWriter::new(zip_file);

    let options: FileOptions<()> = FileOptions::default()
        .compression_method(CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let mut buffer = Vec::new();

    for entry in WalkDir::new(src_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        // Path inside the zip, relative to src_dir
        let name = path.strip_prefix(src_dir).unwrap();

        if path.is_file() {
            zip.start_file(name.to_string_lossy(), options)?;
            let mut f = File::open(path)?;
            f.read_to_end(&mut buffer)?;
            zip.write_all(&buffer)?;
            buffer.clear();
        } else if !name.as_os_str().is_empty() {
            // Add directory entries (needed for empty dirs)
            zip.add_directory(name.to_string_lossy(), options)?;
        }
    }

    zip.finish()?;

    Ok(())
}