use std::io::Error;
use structs::Config;
use terminal::init;

mod structs;
mod terminal;
mod database;
mod config;

fn start() -> Result<Config<'static>, Error> {
    let config = config::config::config_file_exists();
    if config {
        let config = config::config::read_config();
        match config {
            Ok(config) => {
                println!("Config file exists");
                println!("Config: {:?}", config);
                Ok(config)
            },
            Err(e) => {
                println!("Error reading config file: {}", e);
                Err(e)
            }
        }
    } else {
        let creds = terminal::init::startup();
        match creds {
            Ok(creds) => {
                println!("Credentials: {:?}", creds);
                let _ = config::config::init_dirs_and_files();
                let config = config::config::create_user_config(creds);
                match config {
                    Ok(config) => {
                        println!("Config: {:?}", config);
                        let write = config::config::write_config(config.clone());
                        match write {
                            Ok(_) => {
                                println!("Config file written successfully");
                                Ok(config)
                            },
                            Err(e) => {
                                println!("Error writing config file: {}", e);
                                Err(e)
                            }
                        }
                    },
                    Err(e) => {
                        println!("Error creating user config: {}", e);
                        Err(e)
                    }
                }
            },
            Err(e) => {
                println!("Error getting credentials: {}", e);
                Err(e)
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let config = start();
    match config {
        Ok(config) => {
            println!("Config: {:?}", config);
            let creds = config.creds.clone();
            let token = database::auth::signin(creds).await;
            match token {
                Ok(token) => {
                    println!("Token: {}", token);
                },
                Err(e) => {
                    println!("Error signing in: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Error starting: {}", e);
        }
    }
}