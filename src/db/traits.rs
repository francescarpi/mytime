use crate::core::errors::Error;
use crate::core::task::Task;
use chrono::NaiveDate;

pub trait Db {
    fn day_tasks(&self, day: NaiveDate) -> Vec<Task>;
    fn month_tasks(&self, month: u32, year: i32) -> Vec<Task>;
    fn week_tasks(&self, week: u32) -> Vec<Task>;
    fn active_task(&self) -> Result<Task, Error>;
    fn task(&self, id: i64) -> Result<Task, Error>;
    fn stop_task(&self, id: i64) -> Result<Task, Error>;
    fn add_task(&self, desc: String, external_id: Option<String>) -> Result<(), Error>;
    fn change_task_desc(&self, id: i64, desc: String) -> Result<(), Error>;
    fn change_task_external_id(&self, id: i64, external_id: String) -> Result<(), Error>;
    fn reopen_id(&self, id: i64) -> Result<(), Error>;
    fn report_task(&self, id: i64) -> Result<(), Error>;
}
