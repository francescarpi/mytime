use crate::db::Db;
use crate::utils::display::{error, success};

pub struct Reopen {}

impl<'a> Reopen {
    pub fn task(db: &'a dyn Db, id: i64) {
        match db.active_task() {
            Ok(task) => error(format!(
                "There is an active task (task {}). It's not possible to open another one.",
                task.id
            )),
            Err(_) => match db.task(id) {
                Ok(task) => {
                    db.reopen_id(task.id).unwrap();
                    success("Task opened again!".to_string());
                }
                Err(_) => error(format!("The task {} does not exists", id)),
            },
        };
    }
}
