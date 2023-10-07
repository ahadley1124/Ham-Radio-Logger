use std::path::PathBuf;
use std::time::SystemTime;
mod database;
mod config;

fn create_db() -> (String, PathBuf) {
    let path: PathBuf = database::database_directory::create_directory();
    let mut file_name: String = String::new();
    println!("Please enter the file name:");
    std::io::stdin().read_line(&mut file_name).expect("Failed to read line");
    let file_name: &str = file_name.trim();
  
    let file_name: String = if file_name.ends_with(".sqlite3") {
        file_name.to_string()
    } else {
        format!("{}.sqlite3", file_name)
    };
    
  
    let db_path: PathBuf = path.join(file_name.clone());
    println!("Database path: {:?}", db_path);
  
    if let Ok(_) = database::init_db::init_db(db_path.clone()) {
        println!("Database initialized successfully in {:?}", db_path);
    } else {
        println!("Failed to initialize database");
    }
    (file_name, db_path)
}

#[tokio::main]
async fn main() {
    let now = SystemTime::now();
    let (file_name, _dir_path) = create_db();
    let _ = config::config::write_config_file(file_name);
    match now.elapsed() {
        Ok(elapsed) => {
            println!("{}", elapsed.as_secs());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}