use std::{env, path::PathBuf};

pub fn get_current_exec_path() -> PathBuf {
    if let Ok(cargo_dir) = env::var("CARGO_MANIFEST_DIR") {
            let dev_path = PathBuf::from(cargo_dir);
            dev_path
        } else {
            let mut exec_path = env::current_exe().unwrap();
            exec_path.pop();
            exec_path
    }
}   