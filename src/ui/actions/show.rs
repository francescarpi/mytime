use chrono::{Datelike, Duration, Local, NaiveDate};
use clap::{Arg, ArgMatches, Command};
use comfy_table::presets::{NOTHING, UTF8_FULL_CONDENSED};
use comfy_table::*;
use std::collections::HashMap;

use crate::core::config::Config;
use crate::core::task::Task;
use crate::core::utils::dates::to_naive;
use crate::core::utils::formatters::{format_datetime, format_seconds, format_time};
use crate::db::traits::Db;
use crate::ui::traits::Action;

pub struct Show<'a> {
    db: &'a dyn Db,
}

impl<'a> Show<'a> {
    pub fn new(db: &'a dyn Db) -> Self {
        Self { db }
    }

    pub fn today(&self) {
        let today = Local::now().date_naive();
        let tasks = self.db.day_tasks(&today);

        println!("📅 Today ({})", format_seconds(&self.working_time(&tasks)));

        self.print_tables(&tasks, true);
    }

    pub fn week(&self) {
        let week = Local::now().iso_week().week();
        let tasks = self.db.week_tasks(&week);

        println!("📅 Week ({})", format_seconds(&self.working_time(&tasks)));

        self.print_tables(&tasks, false);
    }

    pub fn month(&self) {
        let today = Local::now();
        let tasks = self.db.month_tasks(&today.month(), &today.year());

        println!("📅 Month ({})", format_seconds(&self.working_time(&tasks)));

        self.print_tables(&tasks, false);
    }

    pub fn relative(&self, value: &i64) {
        let today = Local::now().date_naive();
        let date = today - Duration::days(*value);
        self.date(&date);
    }

    pub fn date(&self, date: &NaiveDate) {
        let tasks = self.db.day_tasks(date);
        println!(
            "📅 {} ({})",
            date.format("%a %b %d"),
            format_seconds(&self.working_time(&tasks))
        );

        self.print_tables(&tasks, true);
    }

    pub fn one_task(&self, task: Task) {
        self.print_tasks_table(&vec![task], false);
    }

    fn print_tables(&self, tasks: &Vec<Task>, show_only_time: bool) {
        self.print_tasks_table(&tasks, show_only_time);
        self.print_summary_table(&tasks);
    }

    fn print_tasks_table(&self, tasks: &Vec<Task>, show_only_time: bool) {
        let mut table = self.create_new_table(self.tasks_table_headers());
        let mut previous_day: Option<NaiveDate> = None;
        let mut daily_time_worked = 0;

        for task in tasks {
            let start = if show_only_time {
                format_time(&task.start)
            } else {
                format_datetime(&task.start)
            };

            let end = match task.end.as_ref() {
                Some(date) => {
                    if show_only_time {
                        format_time(&date)
                    } else {
                        format_datetime(&date)
                    }
                }
                None => "🏃".to_string(),
            };

            let reported_color = if task.reported { Color::Green } else { Color::Red };
            let external_id = task
                .external_id
                .as_ref()
                .unwrap_or(&"".to_string())
                .to_owned();

            if previous_day.is_some() && previous_day.unwrap() != to_naive(&task.start) {
                self.add_daily_time_worked_row(&daily_time_worked, &mut table);
                daily_time_worked = 0;
            }

            table.add_row(vec![
                Cell::new(task.id).set_alignment(CellAlignment::Right),
                Cell::new(&task.project),
                Cell::new(&task.desc),
                Cell::new(external_id).set_alignment(CellAlignment::Right),
                Cell::new(start),
                Cell::new(end).set_alignment(CellAlignment::Center),
                Cell::new(format_seconds(&task.duration())).set_alignment(CellAlignment::Right),
                Cell::new("●").set_alignment(CellAlignment::Center).fg(reported_color),
            ]);

            previous_day = Some(to_naive(&task.start));
            daily_time_worked += task.duration();
        }

        self.add_daily_time_worked_row(&daily_time_worked, &mut table);

        println!("{table}");
    }

    fn add_daily_time_worked_row(&self, duration: &i64, table: &mut Table) {
        table.add_row(vec![
            Cell::new(""),
            Cell::new(""),
            Cell::new(""),
            Cell::new(""),
            Cell::new(""),
            Cell::new(""),
            Cell::new(format_seconds(&duration))
                .set_alignment(CellAlignment::Right)
                .add_attribute(Attribute::Bold),
        ]);
    }

    pub fn print_summary_table(&self, tasks: &Vec<Task>) {
        let mut container = Table::new();

        container.load_preset(NOTHING).set_header(vec![
            Cell::new("📚 Grouped by description"),
            Cell::new("📚 Grouped by project"),
        ]);

        container.column_mut(0).unwrap().set_padding((0, 0));
        container.column_mut(1).unwrap().set_padding((3, 0));

        let mut table_group_by_desc =
            self.create_new_table(vec![Cell::new("Description"), Cell::new("Duration")]);

        let mut table_group_by_proj =
            self.create_new_table(vec![Cell::new("Project"), Cell::new("Duration")]);

        let mut grouped_by_desc: HashMap<&String, i64> = HashMap::new();
        let mut grouped_by_proj: HashMap<&String, i64> = HashMap::new();

        for task in tasks {
            let duration_desc = grouped_by_desc.entry(&task.desc).or_insert(0);
            let duration_proj = grouped_by_proj.entry(&task.project).or_insert(0);
            *duration_desc += task.duration();
            *duration_proj += task.duration();
        }

        self.add_rows_in_summary_table(&mut table_group_by_desc, &grouped_by_desc);
        self.add_rows_in_summary_table(&mut table_group_by_proj, &grouped_by_proj);

        container.add_row(vec![table_group_by_desc, table_group_by_proj]);

        println!("{container}");
    }

    fn add_rows_in_summary_table(&self, table: &mut Table, rows: &HashMap<&String, i64>) {
        for (key, value) in rows {
            table.add_row(vec![
                Cell::new(key),
                Cell::new(format_seconds(&value)).set_alignment(CellAlignment::Right),
            ]);
        }
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
            .load_preset(UTF8_FULL_CONDENSED)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(headers);
        table
    }

    fn tasks_table_headers(&self) -> Vec<Cell> {
        vec![
            Cell::new("#").fg(Color::Green),
            Cell::new("Project"),
            Cell::new("Description"),
            Cell::new("Ext.ID"),
            Cell::new("Start"),
            Cell::new("End"),
            Cell::new("Duration"),
            Cell::new("R"),  // Reported
        ]
    }

    fn validate_date(date: &str) -> Result<NaiveDate, String> {
        match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            Ok(date) => Ok(date),
            Err(_) => Err(String::from("Invalid date")),
        }
    }
}

impl Action for Show<'_> {
    const NAME: &'static str = "show";

    fn perform<'a, 'b>(_config: &'b Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let show = Show::new(db);
        if let Some(period) = sub_m.get_one::<String>("period") {
            match period.as_str() {
                "today" => show.today(),
                "week" => show.week(),
                "month" => show.month(),
                _ => show.today(),
            };
        } else if let Some(relative) = sub_m.get_one::<i64>("relative") {
            show.relative(relative);
        } else if let Some(date) = sub_m.get_one::<NaiveDate>("date") {
            show.date(date);
        } else {
            show.today();
        }
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME)
            .about("Show the list of tasks")
            .arg(
                Arg::new("period")
                    .short('p')
                    .long("period")
                    .conflicts_with_all(&["relative", "date"])
                    .value_parser(["today", "week", "month"]),
            )
            .arg(
                Arg::new("relative")
                    .short('r')
                    .long("relative")
                    .conflicts_with_all(&["period", "date"])
                    .help("1 == -1 == yesterday")
                    .value_parser(clap::value_parser!(i64).range(0..=7)),
            )
            .arg(
                Arg::new("date")
                    .short('d')
                    .long("date")
                    .conflicts_with_all(&["relative", "period"])
                    .value_parser(Self::validate_date)
                    .help("Format: YYYY-MM-DD"),
            )
    }
}
