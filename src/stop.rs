use crate::config::Config;
// use crate::task::Task;
// use crate::utils::display::{error, success};
// use crate::utils::formatters::format_seconds;
// use rusqlite::Result;

#[allow(dead_code)]
pub struct Stop<'a> {
    config: &'a Config,
}

impl<'a> Stop<'a> {
    pub fn active(config: &'a Config) -> Self {
        // match Self::active_task(&config) {
        //     Ok(mut task) => {
        //         task.update_duration(&config);
        //         task.stop(&config);
        //
        //         println!("\nTask ID: {}", task.id);
        //         println!("Description: {}", task.desc);
        //         println!("Duration: {}", format_seconds(task.duration));
        //
        //         success("Stopped!".to_string());
        //     }
        //     Err(_) => {
        //         error("There is not any active task!".to_string());
        //     }
        // };
        Self { config: &config }
    }

    // fn active_task(config: &Config) -> Result<Task> {
    //     let mut stmt = config
    //         .conn
    //         .prepare("SELECT * FROM tasks WHERE end_at IS NULL")
    //         .unwrap();
    //
    //     let row = stmt.query_row([], |row| {
    //         Ok(Task {
    //             id: row.get(0)?,
    //             desc: row.get(1)?,
    //             start: row.get(2)?,
    //             end: row.get(3)?,
    //         })
    //     });
    // }
}
