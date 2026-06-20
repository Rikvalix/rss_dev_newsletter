use log::info;
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::{fs, io};

pub fn check_or_create_folder(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

pub fn create_file(file_path: &str, content: &str) -> PathBuf {
    let full_path = Path::new(file_path);
    let mut file = File::create(full_path).expect("Unable to create file");

    file.write_all(content.as_bytes())
        .expect("Unable to write content in file");
    info!("Created file: {}", full_path.display());

    file.seek(SeekFrom::Start(0)).expect("Could not seek file");
    full_path.to_owned()
}
