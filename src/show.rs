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
    }

    pub fn week(&self) {
        println!("📅 Week");
        self.render_table("");
    }

    pub fn month(&self) {
        println!("📅 Month");
        self.render_table("");
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
        let mut table = Table::new();

        let headers = vec![
            Cell::new("#")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green),
            Cell::new("Desc").add_attribute(Attribute::Bold),
            Cell::new("Start").add_attribute(Attribute::Bold),
            Cell::new("End").add_attribute(Attribute::Bold),
            Cell::new("Duration").add_attribute(Attribute::Bold),
        ];

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_width(80)
            .set_header(headers);

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
}
