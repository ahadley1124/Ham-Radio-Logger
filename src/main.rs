fn main() {
    let appdata_dir = std::env::var("APPDATA").unwrap();
    let data_dir = format!("{}/{}", appdata_dir, "ham-radio-logger");
    let config_path = format!("{}/{}", data_dir, "config.toml");
    let _ = std::fs::create_dir_all(data_dir);
    let _ = std::fs::File::create(config_path.clone());
    println!("Config path: {:?}", config_path.clone());
    println!("Config file written successfully in {:?}", config_path);
}