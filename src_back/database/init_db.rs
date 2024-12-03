use sqlite::{Connection, Result};
use std::path::PathBuf;
use std::fs;

pub fn check_db_file(filename: PathBuf) -> bool {
    if filename.exists() {
        println!("Database file already exists. Do you want to delete it? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_lowercase();
        if input == "y" {
            fs::remove_file(&filename).unwrap();
            return false;
        } else {
            let mut new_filename = filename.clone();
            let mut i = 1;
            while new_filename.exists() {
                let filename_str = filename.to_str().unwrap();
                let new_filename_str = filename_str.replace(".sqlite3", &format!("{}.sqlite3", i));
                new_filename = PathBuf::from(new_filename_str);
                i += 1;
            }
            return check_db_file(new_filename);
        }
    } else {
        return false;
    }
}

pub fn create_db(filename: PathBuf) -> Connection {
    let conn = Connection::open(filename).unwrap();
    // close the connection to the database
    conn
}

fn create_callsigns_table(conn: &Connection) -> Result<&Connection> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS callsigns (
            id INTEGER PRIMARY KEY,
            callsign TEXT NOT NULL
        )"
    )?;
    Ok(&conn)
}

fn create_qsl_table(conn: &Connection) -> Result<&Connection> {
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS qsl (
            id INTEGER PRIMARY KEY,
            callsign_id INTEGER NOT NULL,
            qsl_date TEXT NOT NULL,
            qsl_sent INTEGER NOT NULL,
            qsl_rcvd INTEGER NOT NULL,
            qsl_freq TEXT NOT NULL,
            FOREIGN KEY (callsign_id) REFERENCES callsigns(id)
        )"
    );
    Ok(&conn)
}

fn create_name_table(conn: &Connection) -> Result<&Connection> {
    let _ = conn.execute("
        CREATE TABLE IF NOT EXISTS name (
            id INTEGER PRIMARY KEY,
            callsign_id INTEGER NOT NULL,
            name TEXT NOT NULL,
            FOREIGN KEY (callsign_id) REFERENCES callsigns(id)
        )
    ");
    Ok(&conn)
}

fn create_method_table(conn: &Connection) -> Result<&Connection> {
    let _ = conn.execute("
        CREATE TABLE IF NOT EXISTS method (
            id INTEGER PRIMARY KEY,
            callsign_id INTEGER NOT NULL,
            method TEXT NOT NULL,
            FOREIGN KEY (callsign_id) REFERENCES callsigns(id)
        )
    ");
    Ok(&conn)
}

pub fn init_db(filename: PathBuf) -> Result<()> {
    let _file_exists = check_db_file(filename.clone());
    let conn = create_db(filename);
    create_callsigns_table(&conn)?;
    create_qsl_table(&conn)?;
    create_name_table(&conn)?;
    create_method_table(&conn)?;
    Ok(())
}