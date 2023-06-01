use std::fs;
use std::path::PathBuf;

use crate::core::config::Config;
use crate::core::errors::Error;
use crate::core::task::Task;
use crate::db::traits::Db;
use chrono::{NaiveDate, Utc};
use rusqlite::{Connection, Statement};

#[derive(Debug)]
pub struct Sqlite {
    conn: Connection,
}

impl Db for Sqlite {
    fn day_tasks(&self, day: NaiveDate) -> Vec<Task> {
        let day = day.format("%Y-%m-%d").to_string();
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE strftime('%Y-%m-%d', start) = ? ORDER BY id DESC")
            .unwrap();
        self.query_tasks(&mut stmt, day)
    }

    fn month_tasks(&self, month: u32, year: i32) -> Vec<Task> {
        let year_month = format!("{}-{:02}", year, month);
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE strftime('%Y-%m', start) = ? ORDER BY id DESC")
            .unwrap();
        self.query_tasks(&mut stmt, year_month)
    }

    fn week_tasks(&self, week: u32) -> Vec<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE strftime('%W', start) = ? ORDER BY id DESC")
            .unwrap();
        self.query_tasks(&mut stmt, week.to_string())
    }

    fn active_task(&self) -> Result<Task, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE end IS NULL")
            .unwrap();
        match stmt.query_row([], |row| {
            Ok(Task {
                id: row.get(0)?,
                desc: row.get(1)?,
                start: row.get(2)?,
                end: row.get(3)?,
                reported: row.get(4)?,
            })
        }) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn task(&self, id: i64) -> Result<Task, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE id = ?")
            .unwrap();
        match stmt.query_row([id], |row| {
            Ok(Task {
                id: row.get(0)?,
                desc: row.get(1)?,
                start: row.get(2)?,
                end: row.get(3)?,
                reported: row.get(4)?,
            })
        }) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn stop_task(&self, id: i64) -> Result<Task, Error> {
        match self.task(id) {
            Ok(task) => {
                let now = Utc::now().to_rfc3339();
                self.conn
                    .execute(
                        "UPDATE tasks SET end = ? WHERE id = ?",
                        [now, task.id.to_string()],
                    )
                    .unwrap();
                Ok(task)
            }
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn add_task(&self, desc: String) -> Result<(), Error> {
        match self.active_task() {
            Ok(_) => Err(Error::ExistActiveTask {}),
            Err(_) => {
                let now = Utc::now().to_rfc3339();
                self.conn
                    .execute("INSERT INTO tasks (desc, start) VALUES (?, ?)", [desc, now])
                    .unwrap();
                Ok(())
            }
        }
    }

    fn change_task_desc(&self, id: i64, desc: String) -> Result<(), Error> {
        match self.task(id) {
            Ok(_) => {
                self.conn
                    .execute(
                        "UPDATE tasks SET desc = ? WHERE id = ?",
                        [desc, id.to_string()],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn reopen_id(&self, id: i64) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                self.add_task(task.desc)?;
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn report_task(&self, id: i64) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                self.conn
                    .execute(
                        "UPDATE tasks SET reported = ? WHERE id = ?",
                        [!task.reported as i32, id as i32],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }
}

impl Sqlite {
    pub fn new(config: Config) -> Self {
        let conn = Self::create_db_if_not_exist(config.app_share_path.clone());
        Self::migrate(&conn);
        Self { conn }
    }

    fn create_db_if_not_exist(app_share_path: PathBuf) -> Connection {
        let db_path = app_share_path.join("mytime.db");
        let db_path = db_path.to_str().unwrap();

        let created = fs::metadata(&db_path).is_err();
        let conn = Connection::open(&db_path).unwrap();

        const VERSION: &str = env!("CARGO_PKG_VERSION");

        if created {
            conn.execute(
                "CREATE TABLE tasks (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    desc        TEXT NOT NULL,
                    start       INTEGER NOT NULL,
                    end         INTEGER DEFAULT NULL,
                    reported    INTEGER NOT NULL DEFAULT 0
                )",
                (),
            )
            .expect("Table \"tasks\" couldn't been created");
            conn.execute("CREATE TABLE app (version TEXT NOT NULL)", ())
                .expect("Table \"app\" couldn't been created");
            conn.execute("INSERT INTO app VALUES (?)", [VERSION])
                .expect("The version had not been added");
        }

        conn
    }

    fn query_tasks(&self, stmt: &mut Statement, param: String) -> Vec<Task> {
        let rows = stmt
            .query_map([param], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    desc: row.get(1)?,
                    start: row.get(2)?,
                    end: row.get(3)?,
                    reported: row.get(4)?,
                })
            })
            .unwrap();

        let mut tasks: Vec<Task> = Vec::new();
        for row in rows {
            tasks.push(row.unwrap());
        }
        tasks
    }

    fn migrate(conn: &Connection) {
        let app_version: &str = env!("CARGO_PKG_VERSION");
        let migrations = vec![
            ("0.1.0", None),
            (
                "0.1.1",
                Some("ALTER TABLE tasks ADD external_id TEXT DEFAULT NULL"),
            ),
        ];

        let mut stmt_db_version = conn.prepare("SELECT version FROM app").unwrap();
        let db_version: String = stmt_db_version
            .query_row((), |row| Ok(row.get(0)?))
            .unwrap();

        let mut start_migrate = false;

        for migration in migrations {
            if start_migrate {
                if let Some(migration) = migration.1 {
                    conn.execute(migration, ()).unwrap();
                }
            }

            if db_version == migration.0 {
                start_migrate = true;
            }
        }

        conn.execute("UPDATE app SET version = ?", [app_version]).unwrap();
    }
}
