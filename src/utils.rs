pub mod formatters {
    pub fn format_seconds(seconds: i64) -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        format!("{}h{}m", hours, minutes)
    }
}
