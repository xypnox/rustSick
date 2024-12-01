use std::env;
use std::fs;
use std::path::PathBuf;
use std::error::Error;

pub fn read_file(relative_path: &str) -> Result<String, Box<dyn Error>> {
    let root_path = env::current_dir()?;
    let file_path = root_path.join(relative_path);
    
    Ok(fs::read_to_string(file_path)?)
}

// Optional: If you want more control or information about the file path
pub fn get_file_path(relative_path: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root_path = env::current_dir()?;
    Ok(root_path.join(relative_path))
}
