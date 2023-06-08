#[cfg(test)]
mod tests {
    use crate::core::task::Task;
    use crate::core::utils::formatters::{format_seconds, string_to_date};
    use crate::core::utils::grouper::group_tasks_for_the_integration;

    #[test]
    fn test_format_seconds_one_minute() {
        assert_eq!(format_seconds(&60), String::from("0h1m"))
    }

    #[test]
    fn test_format_seconds_one_hour() {
        assert_eq!(format_seconds(&3600), String::from("1h0m"))
    }

    #[test]
    fn test_format_seconds_one_and_half() {
        assert_eq!(format_seconds(&5400), String::from("1h30m"))
    }

    #[test]
    fn test_format_string_to_date() {
        assert_eq!(
            string_to_date(&"2023-06-06T12:00:36.623091+00:00".to_string()),
            String::from("2023-06-06")
        )
    }

    #[test]
    fn test_group_tasks_for_the_integration() {
        let task1 = Task {
            id: 1,
            desc: "Task 1".to_string(),
            start: "2023-06-08T09:51:21.617214+00:00".to_string(),
            end: Some("2023-06-08T10:57:40.751681+00:00".to_string()),
            reported: false,
            external_id: Some("1234".to_string()),
            project: "Proj 1".to_string(),
        };

        let task2 = Task {
            id: 2,
            desc: "Task 1".to_string(),
            start: "2023-06-08T09:51:21.617214+00:00".to_string(),
            end: Some("2023-06-08T10:57:40.751681+00:00".to_string()),
            reported: false,
            external_id: Some("1234".to_string()),
            project: "Proj 1".to_string(),
        };

        let task3 = Task {
            id: 3,
            desc: "Task 2".to_string(),
            start: "2023-06-08T09:51:21.617214+00:00".to_string(),
            end: Some("2023-06-08T10:57:40.751681+00:00".to_string()),
            reported: false,
            external_id: Some("12345".to_string()),
            project: "Proj 1".to_string(),
        };

        let tasks = vec![task1, task2, task3];
        let grouped_tasks = group_tasks_for_the_integration(&tasks);

        assert_eq!(grouped_tasks.len(), 2);
        assert_eq!(grouped_tasks[0].external_id, "1234".to_string());
        assert_eq!(grouped_tasks[0].duration, 7958);
        assert_eq!(grouped_tasks[0].desc, "Task 1".to_string());
        assert_eq!(grouped_tasks[0].start, "2023-06-08".to_string());
        assert_eq!(grouped_tasks[0].ids_used, [1, 2]);

        assert_eq!(grouped_tasks[1].external_id, "12345".to_string());
        assert_eq!(grouped_tasks[1].duration, 3979);
        assert_eq!(grouped_tasks[1].desc, "Task 2".to_string());
        assert_eq!(grouped_tasks[1].start, "2023-06-08".to_string());
        assert_eq!(grouped_tasks[1].ids_used, [3]);

    }
}
