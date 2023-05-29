pub mod formatters {
    use ::chrono::DateTime;

    pub fn format_seconds(seconds: i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h{}m", hours, minutes)
    }

    pub fn format_date(date_time: String) -> String {
        DateTime::parse_from_rfc3339(&date_time)
            .unwrap()
            .format("%Y-%m-%d %H:%M")
            .to_string()
    }
}
