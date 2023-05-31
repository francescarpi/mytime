use std::fs;
use std::path::PathBuf;

use crate::config::Config;
use crate::db::Db;
use crate::errors::Error;
use crate::task::Task;
use chrono::{DateTime, Utc};
use rusqlite::{Connection, Statement};

#[derive(Debug)]
pub struct Sqlite {
    conn: Connection,
}

impl Db for Sqlite {
    fn day_tasks(&self, day: DateTime<Utc>) -> Vec<Task> {
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
            })
        }) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn task(&self, id: u32) -> Result<Task, Error> {
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
            })
        }) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }

    fn stop_task(&self, id: u32) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                let now = Utc::now().to_rfc3339();
                self.conn
                    .execute(
                        "UPDATE tasks SET end = ? WHERE id = ?",
                        [now, task.id.to_string()],
                    )
                    .unwrap();
                Ok(())
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

    fn change_task_desc(&self, id: u32, desc: String) -> Result<(), Error> {
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

    fn reopen_id(&self, id: u32) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                self.add_task(task.desc)?;
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist {}),
        }
    }
}

impl Sqlite {
    pub fn new(config: Config) -> Self {
        Self {
            conn: Self::create_db_if_not_exist(config.app_share_path.clone()),
        }
    }

    fn create_db_if_not_exist(app_share_path: PathBuf) -> Connection {
        let db_path = app_share_path.join("mytime.db");
        let db_path = db_path.to_str().unwrap();

        let created = fs::metadata(&db_path).is_err();
        let conn = Connection::open(&db_path).unwrap();

        if created {
            conn.execute(
                "CREATE TABLE tasks (
                    id      INTEGER PRIMARY KEY AUTOINCREMENT,
                    desc    TEXT NOT NULL,
                    start   INTEGER NOT NULL,
                    end     INTEGER DEFAULT NULL
                )",
                (),
            )
            .unwrap();
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
                })
            })
            .unwrap();

        let mut tasks: Vec<Task> = Vec::new();
        for row in rows {
            tasks.push(row.unwrap());
        }
        tasks
    }
}
