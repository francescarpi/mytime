use crate::core::errors::Error;
use crate::core::task::Task;
use crate::core::todo::Todo;
use chrono::{NaiveDate, NaiveTime};

pub trait Db {
    fn day_tasks(&self, day: &NaiveDate) -> Vec<Task>;
    fn month_tasks(&self, month: &u32, year: &i32) -> Vec<Task>;
    fn week_tasks(&self, week: &u32) -> Vec<Task>;
    fn active_task(&self) -> Result<Task, Error>;
    fn task(&self, id: &i64) -> Result<Task, Error>;
    fn last_task(&self) -> Result<Task, Error>;
    fn stop_task(&self, id: &i64) -> Result<Task, Error>;
    fn add_task(&self, project: &String, desc: &String, external_id: &Option<String>) -> Result<(), Error>;
    fn change_task_desc(&self, id: &i64, desc: &String) -> Result<(), Error>;
    fn change_task_external_id(&self, id: &i64, external_id: &String) -> Result<(), Error>;
    fn change_task_project(&self, id: &i64, project: &String) -> Result<(), Error>;
    fn change_task_start_time(&self, id: &i64, start_time: &NaiveTime) -> Result<(), Error>;
    fn change_task_end_time(&self, id: &i64, end_time: &NaiveTime) -> Result<(), Error>;
    fn reopen_id(&self, id: &i64) -> Result<(), Error>;
    fn report_task(&self, id: &i64) -> Result<(), Error>;
    fn unreported_tasks(&self) -> Vec<Task>;

    fn todo_list(&self) -> Vec<Todo>;
    fn todo_add(&self, project: &String, desc: &String) -> Result<(), Error>;
    fn todo_mark_as_done(&self, id: &i64) -> Result<(), Error>;
    fn todo_detail(&self, id: &i64) -> Result<Todo, Error>;
}
