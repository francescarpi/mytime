use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Task {
    pub id: i64,
    pub desc: String,
    pub start: String,
    pub end: Option<String>,
    pub reported: bool,
    pub external_id: Option<String>,
    pub project: String,
}

impl Task {
    pub fn duration(&self) -> i64 {
        let start = DateTime::parse_from_rfc3339(&self.start).unwrap();
        let end = match self.end.as_ref() {
            Some(end) => DateTime::parse_from_rfc3339(&end)
                .unwrap()
                .with_timezone(&Utc),
            None => Utc::now(),
        };
        end.timestamp() - start.timestamp()
    }

    pub fn is_opened(&self) -> bool {
        self.end.is_none()
    }
}
