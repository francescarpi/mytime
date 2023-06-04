pub mod config;
pub mod errors;
pub mod task;
pub mod utils;

#[cfg(test)]
mod tests {
    use crate::core::utils::formatters::format_seconds;

    #[test]
    fn format_seconds_one_minute() {
        assert_eq!(format_seconds(&60), String::from("0h1m"))
    }

    #[test]
    fn format_seconds_one_hour() {
        assert_eq!(format_seconds(&3600), String::from("1h0m"))
    }

    #[test]
    fn format_seconds_one_and_half() {
        assert_eq!(format_seconds(&5400), String::from("1h30m"))
    }
}
