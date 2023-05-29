use chrono::{DateTime, Utc};
use rusqlite::Connection;
use std::path::Path;
use std::{env, fs};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Config {
    share_path: String,
    db_path: String,
    pub conn: Connection,
    pub now: DateTime<Utc>,
}

impl Config {
    pub fn init() -> Self {
        let home = env::var("HOME").unwrap();

        let share_path = Path::new(&home).join(".local").join("share").join("mytime");
        let db_path = Path::new(&share_path).join("mytime.db");

        let share_path = share_path.to_str().unwrap().to_string();
        let db_path = db_path.to_str().unwrap().to_string();

        Self::create_share_folder_if_not_exist(share_path.clone());
        let conn = Self::create_db_if_not_exist(db_path.clone());

        Self {
            share_path,
            db_path,
            conn,
            now: Utc::now(),
        }
    }

    fn create_share_folder_if_not_exist(share_path: String) {
        if fs::metadata(&share_path).is_err() {
            fs::create_dir_all(&share_path)
                .expect("'mytime' configuration folder has not been created");
        }
    }

    fn create_db_if_not_exist(db_path: String) -> Connection {
        let is_created = fs::metadata(&db_path).is_err();
        let conn = Connection::open(&db_path).unwrap();

        if is_created {
            conn.execute(
                "CREATE TABLE tasks (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    desc        TEXT NOT NULL,
                    start_at    INTEGER NOT NULL,
                    end_at      INTEGER DEFAULT NULL,
                    duration    INTEGER DEFAULT 0
                )",
                (),
            )
            .unwrap();
        }

        conn
    }
}
