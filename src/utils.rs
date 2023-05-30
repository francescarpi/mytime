pub mod formatters {
    use ::chrono::{DateTime, Local};

    pub fn format_seconds(seconds: i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h{}m", hours, minutes)
    }

    pub fn format_date(date_time: String) -> String {
        DateTime::parse_from_rfc3339(&date_time)
            .unwrap()
            .with_timezone(&Local)
            .format("%Y-%m-%d %H:%M")
            .to_string()
    }

    pub fn format_time(date_time: String) -> String {
        DateTime::parse_from_rfc3339(&date_time)
            .unwrap()
            .with_timezone(&Local)
            .format("%H:%M")
            .to_string()
    }
}

pub mod display {
    pub fn success(msg: String) {
        println!("\n✅ {msg}\n");
    }

    pub fn error(msg: String) {
        println!("\n‼️ {msg}\n");
    }
}
