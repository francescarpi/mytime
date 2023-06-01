use crate::core::utils::display::{error, success};
use crate::db::traits::Db;

pub struct Report {}

impl<'a> Report {
    pub fn task(db: &'a dyn Db, id: i64) {
        match db.report_task(id) {
            Ok(_) => success("Task mark as reported!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }
}
