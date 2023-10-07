use std::path::{Path, PathBuf};
use whoami::username;

#[cfg(target_os = "windows")]
pub fn create_directory() -> PathBuf {
    let username = username();
    let dir = format!("C:\\Users\\{}\\Documents\\HamRadioLogger", username);
    let path = Path::new(&dir);
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }
    path.to_path_buf()
}

#[cfg(target_os = "macos")]
pub fn create_directory() -> PathBuf {
    let username = username();
    let dir = format!("/Users/{}/Library/HamRadioLogger", username);
    let path = Path::new(&dir);
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }
    path.to_path_buf()
}

#[cfg(target_os = "linux")]
pub fn create_directory() -> PathBuf {
    let dir = "/etc/HamRadioLogger";
    let path = Path::new(&dir);
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("Failed to create directory");
    }
    path.to_path_buf()
}