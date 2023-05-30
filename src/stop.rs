use crate::config::Config;
use crate::task::Task;
use crate::utils::formatters::format_seconds;
use rusqlite::Result;

#[allow(dead_code)]
pub struct Stop<'a> {
    config: &'a Config,
}

impl<'a> Stop<'a> {

    pub fn active(config: &'a Config) -> Self {
        match Self::active_task(&config) {
            Ok(mut task) => {
                task.update_duration(&config);
                task.stop(&config);

                println!("\nTask ID: {}", task.id);
                println!("Description: {}", task.desc);
                println!("Duration: {}", format_seconds(task.duration));
                println!("✅ Stopped!\n");
            }
            Err(_) => {
                println!("\n‼️ There is not any active task!\n");
            }
        };
        Self { config: &config }
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
