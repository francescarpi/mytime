use crate::config::Config;
use crate::task::Task;
use crate::utils::formatters::{format_date, format_seconds};
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub struct Show<'a> {
    config: &'a Config,
}

impl<'a> Show<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config: &config }
    }

    pub fn today(&self) {
        println!("📅 Today");
        let today = self.config.now.format("%Y-%m-%d").to_string();
        let where_clause = format!(" WHERE strftime('%Y-%m-%d', start_at) = '{}'", today);
        self.render_table(&where_clause);
        self.summary(&where_clause);
    }

    pub fn week(&self) {
        println!("📅 Week");
        let week = self.config.now.format("%V").to_string();
        let where_clause = format!(" WHERE strftime('%W', start_at) = '{}'", week);
        self.render_table(&where_clause);
        self.summary(&where_clause);
    }

    pub fn month(&self) {
        println!("📅 Month");
        let month = self.config.now.format("%Y-%m").to_string();
        let where_clause = format!(" WHERE strftime('%Y-%m', start_at) = '{}'", month);
        self.render_table(&where_clause);
        self.summary(&where_clause);
    }

    fn tasks(&self, where_clause: &str) -> Vec<Task> {
        let query = format!("SELECT * FROM tasks {} ORDER BY id DESC", &where_clause);
        let mut stmt = self.config.conn.prepare(&query).unwrap();

        let rows = stmt
            .query_map([], |row| {
                Ok(Task {
                    id: row.get(0)?,
                    desc: row.get(1)?,
                    start_at: row.get(2)?,
                    end_at: row.get(3)?,
                    duration: row.get(4)?,
                })
            })
            .unwrap();

        let mut tasks: Vec<Task> = Vec::new();
        for row in rows {
            tasks.push(row.unwrap());
        }
        tasks
    }

    fn render_table(&self, where_clause: &str) {
        let mut table = self.table();
        table.set_header(vec![
            Cell::new("#")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green),
            Cell::new("Desc").add_attribute(Attribute::Bold),
            Cell::new("Start").add_attribute(Attribute::Bold),
            Cell::new("End").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
        ]);

        let tasks = self.tasks(&where_clause);
        for mut task in tasks {
            let end_at = match task.end_at {
                Some(date) => format_date(date),
                None => {
                    task.update_duration(&self.config);
                    "🏃".to_string()
                }
            };
            table.add_row(vec![
                Cell::new(task.id),
                Cell::new(task.desc),
                Cell::new(format_date(task.start_at)),
                Cell::new(end_at).set_alignment(CellAlignment::Center),
                Cell::new(format_seconds(task.duration)).set_alignment(CellAlignment::Right),
            ]);
        }

        println!("{table}");
    }

    pub fn summary(&self, where_clause: &str) {
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

        let mut table = self.table();
        table.set_header(vec![
            Cell::new("Desc").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
        ]);

        for row in rows {
            let row = row.unwrap();
            table.add_row(vec![
                Cell::new(row.desc),
                Cell::new(format_seconds(row.duration)),
            ]);
        }

        println!("{table}");
    }

    fn table(&self) -> Table {
        let mut table = Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80);
        table
    }
}
