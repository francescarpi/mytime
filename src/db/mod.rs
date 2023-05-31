use crate::errors::Error;
use crate::task::Task;
use chrono::{DateTime, Utc};

pub mod sqlite;

pub trait Db {
    fn day_tasks(&self, day: DateTime<Utc>) -> Vec<Task>;
    fn month_tasks(&self, month: u32, year: i32) -> Vec<Task>;
    fn week_tasks(&self, week: u32) -> Vec<Task>;
    fn active_task(&self) -> Result<Task, Error>;
    fn task(&self, id: u32) -> Result<Task, Error>;
    fn stop_task(&self, id: u32) -> Result<(), Error>;
    fn add_task(&self, desc: String) -> Result<(), Error>;
    fn change_task_desc(&self, id: u32, desc: String) -> Result<(), Error>;
    fn reopen_id(&self, id: u32) -> Result<(), Error>;

}
