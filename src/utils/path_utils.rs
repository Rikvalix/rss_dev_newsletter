use std::{env, path::PathBuf};

pub fn get_current_exec_path() -> Result<PathBuf, std::io::Error> {
    if let Ok(cargo_dir) = env::var("CARGO_MANIFEST_DIR") {
        let dev_path = PathBuf::from(cargo_dir);
        Ok(dev_path)
    } else {
        let mut exec_path = env::current_exe()?;
        exec_path.pop();
        Ok(exec_path)
    }
}
