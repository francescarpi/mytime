// use crate::{config::Config, db};
use crate::config::Config;
// use crate::start::Start;
// use crate::task::Task;
// use crate::utils::display::error;
// use rusqlite::Result;

pub struct Reopen {}

impl Reopen {
    pub fn task(config: &Config, id: i64) {
        println!("{id}");
        dbg!(config);
        // match Self::get_task(&config, id) {
        //     Ok(task) => {
        //         Start::task(&config, task.desc);
        //     }
        //     Err(_) => {
        //         error("There is not any task with this ID!".to_string());
        //     }
        // }
    }

    // fn get_task(config: &Config, id: i64) -> Result<Task> {
    //     let mut stmt = config
    //         .conn
    //         .prepare("SELECT * FROM tasks WHERE id = ? AND end_at IS NOT NULL")?;
    //     stmt.query_row([id], |row| {
    //         Ok(Task {
    //             id: row.get(0)?,
    //             desc: row.get(1)?,
    //             start_at: row.get(2)?,
    //             end_at: row.get(3)?,
    //             duration: row.get(4)?,
    //         })
    //     })
    // }
}
