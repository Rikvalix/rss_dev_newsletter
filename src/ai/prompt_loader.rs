use crate::utils::path_utils::get_current_exec_path;
use std::{fs, io};

// Load ai system prompt
pub fn load_ai_instruction(file_path: &str) -> io::Result<String> {
    fs::read_to_string(get_ai_directory(&file_path)?)
}

// Load ai user message
pub fn load_ai_message(file_path: &str) -> io::Result<String> {
    fs::read_to_string(get_ai_directory(&file_path)?)
}

fn get_ai_directory(file_path: &str) -> io::Result<String> {
    Ok(format!(
        "{}/config/ai/{}",
        get_current_exec_path()?.to_string_lossy(),
        file_path
    ))
}
