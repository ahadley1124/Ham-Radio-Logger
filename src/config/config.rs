use std::io::Error;

use crate::structs::{Config, Credentials};

pub fn init_dirs_and_files() -> Result<(), Error> {
    let appdata_dir = std::env::var("APPDATA").unwrap();
    let data_dir = format!("{}/{}", appdata_dir, "ham-radio-logger");
    let config_path = format!("{}/{}", data_dir, "config.json");
    let _ = std::fs::create_dir_all(data_dir);
    let _ = std::fs::File::create(config_path.clone());
    println!("Config path: {:?}", config_path.clone());
    println!("Config file written successfully in {:?}", config_path);
    Ok(())
}

pub fn create_user_config(creds: Credentials<'static>) -> Result<Config<'static>, Error> {
    println!("Enter your Grid Square: ");
    let mut grid = String::new();
    std::io::stdin().read_line(&mut grid).unwrap();
    println!("Enter your QTH: ");
    let mut qth = String::new();
    std::io::stdin().read_line(&mut qth).unwrap();
    println!("Enter your Rig: ");
    let mut rig = String::new();
    std::io::stdin().read_line(&mut rig).unwrap();
    println!("Enter your Power: ");
    let mut power = String::new();
    std::io::stdin().read_line(&mut power).unwrap();
    println!("Enter your Antenna: ");
    let mut antenna = String::new();
    std::io::stdin().read_line(&mut antenna).unwrap();
    Ok(Config {
        creds: creds,
        grid: grid.trim().to_string(),
        qth: qth.trim().to_string(),
        rig: rig.trim().to_string(),
        power: power.trim().to_string(),
        antenna: antenna.trim().to_string(),
    })
}

pub fn config_file_exists() -> bool {
    let appdata_dir = std::env::var("APPDATA").unwrap();
    let data_dir = format!("{}/{}", appdata_dir, "ham-radio-logger");
    let config_path = format!("{}/{}", data_dir, "config.json");
    std::path::Path::new(&config_path).exists()
}

pub fn read_config() -> Result<Config<'static>, Error> {
    todo!("Impliment a function to read the config file");
}

pub fn write_config(config: Config) -> Result<(), Error> {
    todo!("Impliment a function to write the config file");
}