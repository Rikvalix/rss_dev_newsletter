use crate::utils::path_utils::get_current_exec_path;
use std::fs;

// Load ai system prompt
pub fn load_ai_instruction(file_path: &str) -> String {
    fs::read_to_string(get_ai_directory(&file_path))
        .unwrap_or_else(|_| panic!("Failed to load ai instruction from {}", file_path))
}

// Load ai user message
pub fn load_ai_message(file_path: &str) -> String {
    fs::read_to_string(get_ai_directory(&file_path))
        .unwrap_or_else(|_| panic!("Failed to load ai message from {}", file_path))
}

fn get_ai_directory(file_path: &str) -> String {
    format!(
        "{}/config/ai/{}",
        get_current_exec_path().to_str().unwrap(),
        file_path
    )
}
