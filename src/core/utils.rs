pub mod formatters {
    use ::chrono::{DateTime, Local};

    pub fn format_seconds(seconds: &i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h{}m", hours, minutes)
    }

    pub fn format_datetime(date_time: &String) -> String {
        DateTime::parse_from_rfc3339(&date_time)
            .unwrap()
            .with_timezone(&Local)
            .format("%a %b %d %H:%M")
            .to_string()
    }

    pub fn format_time(date_time: &String) -> String {
        DateTime::parse_from_rfc3339(&date_time)
            .unwrap()
            .with_timezone(&Local)
            .format("%H:%M")
            .to_string()
    }

    pub fn string_to_date(date: &String) -> String {
        DateTime::parse_from_rfc3339(&date)
            .unwrap()
            .with_timezone(&Local)
            .to_string()
    }
}

pub mod display {
    pub fn success(msg: String) {
        println!("✅ {msg}");
    }

    pub fn error(msg: String) {
        println!("‼️ {msg}");
    }
}

pub mod dates {
    use chrono::{DateTime, NaiveDate};

    pub fn to_naive(date: &String) -> NaiveDate {
        DateTime::parse_from_rfc3339(date).unwrap().date_naive()
    }
}
