use crate::config::Config;
use chrono::DateTime;

#[derive(Debug)]
pub struct Task {
    pub id: i64,
    pub desc: String,
    pub start_at: String,
    pub end_at: Option<String>,
    pub duration: i64,
}

impl Task {
    pub fn stop(&self, config: &Config) {
        config
            .conn
            .execute(
                "UPDATE tasks SET end_at = ?1 WHERE id = ?2",
                [config.now.to_rfc3339(), self.id.to_string()],
            )
            .unwrap();
    }

    pub fn update_duration(&mut self, config: &Config) {
        let start_at = DateTime::parse_from_rfc3339(&self.start_at).unwrap();
        let duration = config.now.timestamp() - start_at.timestamp();
        config
            .conn
            .execute(
                "UPDATE tasks SET duration = ?1 WHERE id = ?2",
                [duration, self.id],
            )
            .unwrap();
        self.duration = duration;
    }
}
