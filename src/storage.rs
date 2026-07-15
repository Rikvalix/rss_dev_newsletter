use log::info;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn check_or_create_folder(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn create_file(file_path: &str, content: &str) -> io::Result<PathBuf> {
    let full_path = Path::new(file_path);
    let mut file = File::create(full_path)?;

    file.write_all(content.as_bytes())?;
    info!("Created file: {}", full_path.display());
    Ok(full_path.to_path_buf())
}
