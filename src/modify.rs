use crate::config::Config;
use crate::utils::display::{error, success};

pub struct Modify {}

impl Modify {
    pub fn task(config: &Config, id: i64, desc: String) {
        match config.conn.execute(
            "UPDATE tasks SET desc = ?1 WHERE id = ?2",
            [desc, id.to_string()],
        ) {
            Ok(_) => {
                success("Task updated!".to_string());
            }
            Err(_) => {
                error("There is not any task with this ID!".to_string());
            }
        }
    }
}
