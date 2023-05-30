use crate::config::Config;
use crate::task::Task;
use crate::utils::formatters::{format_date, format_seconds, format_time};
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;
use rusqlite::Result;

pub struct Show<'a> {
    config: &'a Config,
}

impl<'a> Show<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config: &config }
    }

    pub fn today(&self) {
        let today = self.config.now.format("%Y-%m-%d").to_string();
        let where_clause = format!(" WHERE strftime('%Y-%m-%d', start_at) = '{}'", today);

        println!("\n📅 Today");

        self.print_tasks_table(&where_clause, true);
        self.print_summary_table(&where_clause);
        self.print_working_time(&where_clause);
    }

    pub fn week(&self) {
        let week = self.config.now.format("%V").to_string();
        let where_clause = format!(" WHERE strftime('%W', start_at) = '{}'", week);

        println!("\n📅 Week");

        self.print_tasks_table(&where_clause, false);
        self.print_summary_table(&where_clause);
        self.print_working_time(&where_clause);
    }

    pub fn month(&self) {
        let month = self.config.now.format("%Y-%m").to_string();
        let where_clause = format!(" WHERE strftime('%Y-%m', start_at) = '{}'", month);

        println!("\n📅 Month");

        self.print_tasks_table(&where_clause, false);
        self.print_summary_table(&where_clause);
        self.print_working_time(&where_clause);
    }

    fn get_tasks_list(&self, where_clause: &str) -> Vec<Result<Task>> {
        let query = format!("SELECT * FROM tasks {} ORDER BY id DESC", &where_clause);
        let mut stmt = self.config.conn.prepare(&query).unwrap();

        stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                desc: row.get(1)?,
                start_at: row.get(2)?,
                end_at: row.get(3)?,
                duration: row.get(4)?,
            })
        })
        .unwrap()
        .collect::<Vec<Result<Task>>>()
    }

    fn print_tasks_table(&self, where_clause: &str, show_only_time: bool) {
        let mut table = self.create_new_table(self.tasks_table_headers());

        for task in self.get_tasks_list(&where_clause) {
            let mut task = task.unwrap();

            let start_at = if show_only_time {
                format_time(task.start_at.clone())
            } else {
                format_date(task.start_at.clone())
            };

            let end_at = match task.end_at {
                Some(date) => {
                    if show_only_time {
                        format_time(date)
                    } else {
                        format_date(date)
                    }
                }
                None => {
                    task.update_duration(&self.config);
                    "🏃".to_string()
                }
            };

            table.add_row(vec![
                Cell::new(task.id),
                Cell::new(task.desc),
                Cell::new(start_at),
                Cell::new(end_at).set_alignment(CellAlignment::Center),
                Cell::new(format_seconds(task.duration)).set_alignment(CellAlignment::Right),
            ]);
        }

        println!("{table}");
    }

    pub fn print_summary_table(&self, where_clause: &str) {
        println!("\n📚 Group by description");
        let query = format!(
            "SELECT desc, SUM(duration) AS duration FROM tasks {} GROUP BY DESC",
            &where_clause
        );
        let mut stmt = self.config.conn.prepare(&query).unwrap();

        #[derive(Debug)]
        struct AggregatedTask {
            desc: String,
            duration: i64,
        }

        let rows = stmt
            .query_map([], |row| {
                Ok(AggregatedTask {
                    desc: row.get(0)?,
                    duration: row.get(1)?,
                })
            })
            .unwrap();

        let mut table = self.create_new_table(self.summary_table_headers());

        for row in rows {
            let row = row.unwrap();
            table.add_row(vec![
                Cell::new(row.desc),
                Cell::new(format_seconds(row.duration)),
            ]);
        }

        println!("{table}");
    }

    pub fn print_working_time(&self, where_clause: &str) {
        let query = format!(
            "SELECT SUM(duration) AS duration FROM tasks {}",
            &where_clause
        );
        let mut stmt = self.config.conn.prepare(&query).unwrap();
        let duration = stmt.query_row([], |row| Ok(row.get(0)?)).unwrap_or(0);

        println!("\n⏱️ Work time: {}\n", format_seconds(duration));
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
            Cell::new("Start").add_attribute(Attribute::Bold),
            Cell::new("End").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
        ]
    }

    fn summary_table_headers(&self) -> Vec<Cell> {
        vec![
            Cell::new("Desc").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
        ]
    }
}
