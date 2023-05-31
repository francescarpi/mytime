use crate::db::Db;
use crate::utils::display::{error, success};
use crate::utils::formatters::format_seconds;

pub struct Stop {}

impl<'a> Stop {
    pub fn active(db: &'a dyn Db) -> Self {
        match db.active_task() {
            Ok(task) => {

                let task = db.stop_task(task.id).unwrap();

                println!("\nTask ID: {}", task.id);
                println!("Description: {}", task.desc);
                println!("Duration: {}", format_seconds(task.duration()));

                success("Stopped!".to_string());
            }
            Err(_) => error("There is not any active task!".to_string()),
        };

        Self {}
    }
}
