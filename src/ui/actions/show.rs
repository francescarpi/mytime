use std::collections::HashMap;

use crate::core::task::Task;
use crate::core::utils::formatters::{format_date, format_seconds, format_time};
use crate::db::traits::Db;
use chrono::{Datelike, Duration, Local, NaiveDate};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub struct Show<'a> {
    db: &'a dyn Db,
}

impl<'a> Show<'a> {
    pub fn new(db: &'a dyn Db) -> Self {
        Self { db }
    }

    pub fn today(&self) {
        let today = Local::now().date_naive();
        let tasks = self.db.day_tasks(today);

        println!("\n📅 Today ({})", format_seconds(self.working_time(&tasks)));

        self.print_tables(&tasks, true);
    }

    pub fn week(&self) {
        let week = Local::now().iso_week().week();
        let tasks = self.db.week_tasks(week);

        println!("\n📅 Week ({})", format_seconds(self.working_time(&tasks)));

        self.print_tables(&tasks, false);
    }

    pub fn month(&self) {
        let today = Local::now();
        let tasks = self.db.month_tasks(today.month(), today.year());

        println!("\n📅 Month ({})", format_seconds(self.working_time(&tasks)));

        self.print_tables(&tasks, false);
    }

    pub fn relative(&self, value: i64) {
        let today = Local::now().date_naive();
        let date = today - Duration::days(value);
        self.date(date);
    }

    pub fn date(&self, date: NaiveDate) {
        let tasks = self.db.day_tasks(date);
        println!(
            "\n📅 Date {} ({})",
            date.format("%Y-%m-%d"),
            format_seconds(self.working_time(&tasks))
        );

        self.print_tables(&tasks, true);
    }

    fn print_tables(&self, tasks: &Vec<Task>, show_only_time: bool) {
        self.print_tasks_table(&tasks, show_only_time);
        self.print_summary_table(&tasks);
    }

    fn print_tasks_table(&self, tasks: &Vec<Task>, show_only_time: bool) {
        let mut table = self.create_new_table(self.tasks_table_headers());

        for task in tasks {
            let start = if show_only_time {
                format_time(task.start.clone())
            } else {
                format_date(task.start.clone())
            };

            let end = match task.end.clone() {
                Some(date) => {
                    if show_only_time {
                        format_time(date)
                    } else {
                        format_date(date)
                    }
                }
                None => "🏃".to_string(),
            };

            let reported = if task.reported { "🟢" } else { "🔴" };

            let external_id = match task.external_id.clone() {
                Some(id) => id,
                None => "".to_string(),
            };

            table.add_row(vec![
                Cell::new(task.id),
                Cell::new(&task.desc),
                Cell::new(external_id).set_alignment(CellAlignment::Right),
                Cell::new(start),
                Cell::new(end).set_alignment(CellAlignment::Center),
                Cell::new(format_seconds(task.duration())).set_alignment(CellAlignment::Right),
                Cell::new(&reported).set_alignment(CellAlignment::Center),
            ]);
        }

        println!("{table}");
    }

    pub fn print_summary_table(&self, tasks: &Vec<Task>) {
        println!("\n📚 Grouped by description");

        let mut table = self.create_new_table(self.summary_table_headers());
        let mut grouped_tasks: HashMap<String, i64> = HashMap::new();

        for task in tasks {
            let duration_sum = grouped_tasks.entry(task.desc.clone()).or_insert(0);
            *duration_sum += task.duration();
        }

        for (desc, duration) in grouped_tasks {
            table.add_row(vec![Cell::new(desc), Cell::new(format_seconds(duration))]);
        }

        println!("{table}");
    }

    pub fn working_time(&self, tasks: &Vec<Task>) -> i64 {
        let mut duration = 0;
        for task in tasks {
            duration += task.duration();
        }
        duration
    }

    fn create_new_table(&self, headers: Vec<Cell>) -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(headers);
        table
    }

    fn tasks_table_headers(&self) -> Vec<Cell> {
        vec![
            Cell::new("#")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green),
            Cell::new("Desc").add_attribute(Attribute::Bold),
            Cell::new("Ext.ID").add_attribute(Attribute::Bold),
            Cell::new("Start").add_attribute(Attribute::Bold),
            Cell::new("End").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
            Cell::new("Reported").add_attribute(Attribute::Bold),
        ]
    }

    fn summary_table_headers(&self) -> Vec<Cell> {
        vec![
            Cell::new("Desc").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
        ]
    }
}
