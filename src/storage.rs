use log::info;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

pub fn check_or_create_folder(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn create_file(file_path: &str, content: &str) -> io::Result<()> {
    let full_path = Path::new(file_path);
    let mut file = fs::File::create(full_path)?;
    file.write_all(content.as_bytes())?;
    info!("Created file: {}", full_path.display());
    Ok(())
}
