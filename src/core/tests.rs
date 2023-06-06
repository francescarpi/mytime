#[cfg(test)]
mod tests {
    use crate::core::utils::formatters::{format_seconds, string_to_date};

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

    #[test]
    fn format_string_to_date() {
        assert_eq!(
            string_to_date(&"2023-06-06T12:00:36.623091+00:00".to_string()),
            String::from("2023-06-06")
        )
    }
}
