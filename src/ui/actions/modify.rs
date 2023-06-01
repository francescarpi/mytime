use crate::db::traits::Db;
use crate::core::utils::display::{error, success};

pub struct Modify {}

impl<'a> Modify {
    pub fn desc(db: &'a dyn Db, id: i64, desc: String) {
        match db.change_task_desc(id, desc) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }

    pub fn external_id(db: &'a dyn Db, id: i64, external_id: String) {
        match db.change_task_external_id(id, external_id) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }
}
