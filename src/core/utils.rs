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
            .format("%Y-%m-%d")
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

pub mod grouper {
    use crate::core::task::Task;
    use crate::integrations::IntegrationTask;
    use chrono::DateTime;
    use std::collections::HashMap;

    pub fn group_tasks_for_the_integration(tasks: &Vec<Task>) -> Vec<IntegrationTask> {
        let mut group: HashMap<String, IntegrationTask> = HashMap::new();

        for task in tasks {
            let desc = &task.desc;
            let start = DateTime::parse_from_rfc3339(&task.start)
                .unwrap()
                .format("%Y-%m-%d")
                .to_string();
            let external_id = task.external_id.as_ref().unwrap().to_owned();
            let project = &task.project;

            let key = format!("{}-{}-{}-{}", desc, start, external_id, project);

            let grouped_task = group.entry(key).or_insert(IntegrationTask {
                external_id,
                duration: 0,
                desc: desc.clone(),
                start,
                ids_used: Vec::new(),
            });

            grouped_task.duration += task.duration();
            grouped_task.ids_used.push(task.id);
        }

        let mut tasks: Vec<IntegrationTask> = group.into_iter().map(|(_k, task)| task).collect();
        tasks.sort();
        tasks
    }

    pub fn group_by_project(tasks: &Vec<Task>) -> HashMap<&String, i64> {
        let mut group: HashMap<&String, i64> = HashMap::new();

        for task in tasks {
            let duration = group.entry(&task.project).or_insert(0);
            *duration += task.duration();
        }

        group
    }
}
