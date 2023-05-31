use crate::db::Db;
use crate::core::utils::display::{error, success};

pub struct Start {}

impl<'a> Start {
    pub fn task(db: &'a dyn Db, desc: String) {
        match db.add_task(desc) {
            Ok(_) => success("Task added successfully!".to_string()),
            Err(_) => {
                error("There is another active task. You have to stop it before.".to_string())
            }
        }
    }
}
