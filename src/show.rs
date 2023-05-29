use crate::config::Config;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

pub struct Show {
    config: Config,
}

impl Show {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn today(&self) {
        println!("📅 Today");
        self.render_table();
    }

    pub fn week(&self) {
        println!("📅 Week");
        self.render_table();
    }

    pub fn month(&self) {
        println!("📅 Month");
        self.render_table();
    }

    fn render_table(&self) {
        let mut table = Table::new();

        let headers = vec![
            Cell::new("#").add_attribute(Attribute::Bold).fg(Color::Green),
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

        println!("{table}");
    }
}
