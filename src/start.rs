use crate::config::Config;
use crate::utils::display::{error, success};

#[allow(dead_code)]
pub struct Start<'a> {
    config: &'a Config,
}

impl<'a> Start<'a> {
    pub fn task(config: &'a Config, desc: String) -> Self {
        if Self::exist_active_task(&config) {
            error("There is another active task. You have to stop it before.".to_string());
        } else {
            config
                .conn
                .execute(
                    "INSERT INTO tasks (desc, start_at) VALUES (?1, ?2)",
                    [desc.clone(), config.now.to_rfc3339()],
                )
                .unwrap();
            success(format!("Task \"{}\" added!", desc));
        }
        Self { config: &config }
    }

    fn exist_active_task(config: &Config) -> bool {
        let mut stmt = config
            .conn
            .prepare("SELECT id FROM tasks WHERE end_at IS NULL LIMIT 1")
            .unwrap();

        stmt.query_row([], |_| Ok(())).is_ok()
    }
}
