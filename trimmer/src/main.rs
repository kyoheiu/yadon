use std::{
    path::PathBuf,
    time::{Duration, UNIX_EPOCH},
};

const DB_PATH: &str = "../server/databases/.sqlite";
const DURATION: u64 = 10;

fn main() {
    let predefined_db_path = PathBuf::from(DB_PATH);
    if !predefined_db_path.exists() {
        std::fs::create_dir_all(predefined_db_path).unwrap();
    }

    let db_path = match std::env::var("DATABASE_PATH") {
        Ok(p) => PathBuf::from(&p),
        Err(_) => PathBuf::from(DB_PATH),
    };
    let connection = sqlite::Connection::open_with_full_mutex(&db_path).unwrap();
    connection.execute(state_create_table()).unwrap();

    //trim the expired columns
    connection.execute(state_trim()).unwrap();
}

fn state_create_table() -> String {
    "
     CREATE TABLE IF NOT EXISTS bin (
     id TEXT PRIMARY KEY,
     body TEXT,
     timestamp DATETIME
     )
    "
    .to_owned()
}

fn state_trim() -> String {
    let now = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    println!("{}", now);
    let duration = now + DURATION;

    format!(
        "
         DELETE FROM bin
         WHERE timestamp > {};
        ",
        duration
    )
}
