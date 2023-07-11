use std::fs;
use std::path::PathBuf;

use crate::core::config::Config;
use crate::core::errors::Error;
use crate::core::task::Task;
use crate::core::todo::Todo;
use crate::core::utils::dates::update_time;
use crate::db::traits::Db;
use chrono::{NaiveDate, NaiveTime, Utc};
use rusqlite::{params, params_from_iter, Connection, Result, Row, Statement};

#[derive(Debug)]
pub struct Sqlite {
    conn: Connection,
}

impl Db for Sqlite {
    fn todo_detail(&self, id: &i64) -> std::result::Result<Todo, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM todo WHERE id = ?")
            .unwrap();
        match stmt.query_row(params![id], |row| self.row_to_todo(row)) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TodoDoesNotExist),
        }
    }

    fn todo_mark_as_done(&self, id: &i64) -> std::result::Result<(), Error> {
        match self.todo_detail(&id) {
            Ok(_todo) => {
                self.conn
                    .execute("DELETE FROM todo WHERE id = ?", params![id])
                    .unwrap();
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    fn todo_add(&self, project: &String, desc: &String) -> std::result::Result<(), Error> {
        let now = Utc::now().to_rfc3339();
        self.conn
            .execute(
                "INSERT INTO todo (project, desc, created) VALUES (?, ?, ?)",
                params![project, desc, now],
            )
            .unwrap();
        Ok(())
    }

    fn todo_list(&self) -> Vec<Todo> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM todo ORDER BY created DESC")
            .unwrap();

        let rows = stmt.query_map((), |row| self.row_to_todo(row)).unwrap();

        let mut todo: Vec<Todo> = Vec::new();
        for row in rows {
            todo.push(row.unwrap());
        }
        todo
    }

    fn day_tasks(&self, day: &NaiveDate) -> Vec<Task> {
        let day = day.format("%Y-%m-%d").to_string();
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE strftime('%Y-%m-%d', start) = ? ORDER BY id DESC")
            .unwrap();
        self.query_tasks(&mut stmt, Some(day))
    }

    fn month_tasks(&self, month: &u32, year: &i32) -> Vec<Task> {
        let year_month = format!("{}-{:02}", year, month);
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE strftime('%Y-%m', start) = ? ORDER BY id DESC")
            .unwrap();
        self.query_tasks(&mut stmt, Some(year_month))
    }

    fn week_tasks(&self, week: &u32) -> Vec<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE strftime('%W', start) = ? ORDER BY id DESC")
            .unwrap();
        self.query_tasks(&mut stmt, Some(week.to_string()))
    }

    fn active_task(&self) -> Result<Task, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE end IS NULL")
            .unwrap();
        match stmt.query_row([], |row| self.row_to_task(row)) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn task(&self, id: &i64) -> Result<Task, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE id = ?")
            .unwrap();
        match stmt.query_row([id], |row| self.row_to_task(row)) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn last_task(&self) -> Result<Task, Error> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks ORDER BY id DESC LIMIT 1")
            .unwrap();
        match stmt.query_row([], |row| self.row_to_task(row)) {
            Ok(task) => Ok(task),
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn stop_task(&self, id: &i64) -> Result<Task, Error> {
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
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn add_task(
        &self,
        project: &String,
        desc: &String,
        external_id: &Option<String>,
    ) -> Result<(), Error> {
        match self.active_task() {
            Ok(_) => Err(Error::ExistActiveTask),
            Err(_) => {
                let now = Utc::now().to_rfc3339();
                self.conn
                    .execute(
                        "INSERT INTO tasks (project, desc, start, external_id) VALUES (?, ?, ?, ?)",
                        params![project, desc, now, Some(external_id)],
                    )
                    .unwrap();
                Ok(())
            }
        }
    }

    fn change_task_desc(&self, id: &i64, desc: &String) -> Result<(), Error> {
        match self.task(id) {
            Ok(_) => {
                self.conn
                    .execute(
                        "UPDATE tasks SET desc = ? WHERE id = ?",
                        [desc, &id.to_string()],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn change_task_project(&self, id: &i64, project: &String) -> Result<(), Error> {
        match self.task(id) {
            Ok(_) => {
                self.conn
                    .execute(
                        "UPDATE tasks SET project = ? WHERE id = ?",
                        [project, &id.to_string()],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn change_task_external_id(&self, id: &i64, external_id: &String) -> Result<(), Error> {
        match self.task(id) {
            Ok(_) => {
                self.conn
                    .execute(
                        "UPDATE tasks SET external_id = ? WHERE id = ?",
                        [external_id, &id.to_string()],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn change_task_start_time(&self, id: &i64, start_time: &NaiveTime) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                let new_datetime = update_time(&task.start, start_time);
                self.conn
                    .execute(
                        "UPDATE tasks SET start = ? WHERE id = ?",
                        params![new_datetime, &id.to_string()],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn change_task_end_time(&self, id: &i64, end_time: &NaiveTime) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => match task.end {
                Some(end_datetime) => {
                    let new_datetime = update_time(&end_datetime, end_time);
                    self.conn
                        .execute(
                            "UPDATE tasks SET end = ? WHERE id = ?",
                            params![new_datetime, &id.to_string()],
                        )
                        .unwrap();
                    Ok(())
                }
                None => Err(Error::TaskDoesNotHaveEndDate),
            },
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn reopen_id(&self, id: &i64) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                self.add_task(&task.project, &task.desc, &task.external_id)?;
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn report_task(&self, id: &i64) -> Result<(), Error> {
        match self.task(id) {
            Ok(task) => {
                self.conn
                    .execute(
                        "UPDATE tasks SET reported = ? WHERE id = ?",
                        params![!task.reported, id],
                    )
                    .unwrap();
                Ok(())
            }
            Err(_) => Err(Error::TaskDoesNotExist),
        }
    }

    fn unreported_tasks(&self) -> Vec<Task> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM tasks WHERE end IS NOT NULL AND reported = false AND external_id IS NOT NULL AND external_id != ''")
            .unwrap();
        self.query_tasks(&mut stmt, None)
    }
}

impl Sqlite {
    pub fn new(config: &Config) -> Self {
        let conn = Self::create_db_if_not_exist(&config.app_share_path);
        Self::migrate(&conn);
        Self { conn }
    }

    fn row_to_task(&self, row: &Row) -> Result<Task> {
        Ok(Task {
            id: row.get(0)?,
            desc: row.get(1)?,
            start: row.get(2)?,
            end: row.get(3)?,
            reported: row.get(4)?,
            external_id: row.get(5)?,
            project: row.get(6)?,
        })
    }

    fn row_to_todo(&self, row: &Row) -> Result<Todo> {
        Ok(Todo {
            id: row.get(0)?,
            project: row.get(1)?,
            desc: row.get(2)?,
            created: row.get(3)?,
        })
    }

    fn create_db_if_not_exist(app_share_path: &PathBuf) -> Connection {
        let db_path = app_share_path.join("mytime.db");
        let db_path = db_path.to_str().unwrap();

        let created = fs::metadata(&db_path).is_err();
        let conn = Connection::open(&db_path).unwrap();

        const VERSION: &str = env!("CARGO_PKG_VERSION");

        if created {
            conn.execute(
                "CREATE TABLE tasks (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    desc            TEXT NOT NULL,
                    start           INTEGER NOT NULL,
                    end             INTEGER DEFAULT NULL,
                    reported        INTEGER NOT NULL DEFAULT 0,
                    external_id     TEXT DEFAULT NULL,
                    project         TEXT NOT NULL
                )",
                (),
            )
            .expect("Table 'tasks' couldn't been created");
            conn.execute("CREATE TABLE app (version TEXT NOT NULL)", ())
                .expect("Table 'app' couldn't been created");
            conn.execute("INSERT INTO app VALUES (?)", [VERSION])
                .expect("The version had not been added");
            conn.execute(
                "CREATE TABLE todo (
                    id      INTEGER PRIMARY KEY AUTOINCREMENT,
                    project TEXT NOT NULL,
                    desc    TEXT NOT NULL,
                    created INTEGER NOT NULL
                )",
                (),
            )
            .expect("Table 'tasks' couldn't been created");
        }

        conn
    }

    fn query_tasks(&self, stmt: &mut Statement, param: Option<String>) -> Vec<Task> {
        let mut params: Vec<String> = Vec::new();
        if param.is_some() {
            params.push(param.unwrap());
        }

        let rows = stmt
            .query_map(params_from_iter(params), |row| self.row_to_task(row))
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
            (
                "0.1.2",
                Some("ALTER TABLE tasks ADD project TEXT NOT NULL DEFAULT ''"),
            ),
            ("0.1.3", None),
            ("0.1.4", None),
            ("0.1.5", Some("CREATE TABLE todo (id INTEGER PRIMARY KEY AUTOINCREMENT, project TEXT NOT NULL, desc TEXT NOT NULL, created INTEGER NOT NULL)")),
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

        conn.execute("UPDATE app SET version = ?", [app_version])
            .unwrap();
    }
}
