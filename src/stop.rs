use crate::config::Config;
use crate::task::Task;
use crate::utils::formatters::format_seconds;
use rusqlite::Result;

pub struct Stop {}

impl Stop {
    pub fn active(config: Config) {
        match Self::active_task(&config) {
            Ok(mut task) => {
                task.update_duration(&config);
                task.stop(&config);

                println!("Task ID: {}", task.id);
                println!("Description: {}", task.desc);
                println!("Duration: {}", format_seconds(task.duration));
                println!("✅ Stopped!");
            }
            Err(_) => {
                println!("‼️ There is not any active task!");
            }
        };
    }

    fn active_task(config: &Config) -> Result<Task> {
        let mut stmt = config
            .conn
            .prepare("SELECT * FROM tasks WHERE end_at IS NULL")
            .unwrap();

        stmt.query_row([], |row| {
            Ok(Task {
                id: row.get(0)?,
                desc: row.get(1)?,
                start_at: row.get(2)?,
                end_at: row.get(3)?,
                duration: row.get(4)?,
            })
        })
    }
}
