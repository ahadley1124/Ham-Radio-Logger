use std::fs::File as file;
use std::io::Write;
use std::path::PathBuf;
use serde_json::{ json, to_string_pretty };
use crate::database::database_directory;

pub fn write_config_file(file_name: String) -> Result<(), Box<dyn std::error::Error>>{
    let path: PathBuf = database_directory::create_directory();
    let dir: PathBuf = path.join("config.json");
    let config_file = file::create(dir);
    let mut config_file = match config_file {
        Ok(file) => file,
        Err(e) => panic!("Failed to create config file: {}", e),
    };
    let config = json!({
        "database": file_name,
        "directory": path
    });
    let config = to_string_pretty(&config)?;
    match config_file.write_all(config.as_bytes()) {
        Ok(_) => println!("Config file created successfully"),
        Err(e) => panic!("Failed to write to config file: {}", e),
    };
    Ok(())
}