use clap::ArgMatches;

use crate::db::traits::Db;
use crate::core::utils::display::{error, success};
use crate::core::utils::formatters::format_seconds;
use crate::ui::actions::traits::Action;
use crate::ui::actions::show::Show;

pub struct Stop {}

impl Action for Stop {
    fn perform<'a>(db: &'a dyn Db, _sub_m: &ArgMatches) {
        match db.active_task() {
            Ok(task) => {

                let task = db.stop_task(task.id).unwrap();

                println!("\nTask ID: {}", task.id);
                println!("Description: {}", task.desc);
                println!("Duration: {}", format_seconds(task.duration()));

                success("Stopped!".to_string());
            }
            Err(_) => error("There is not any active task!".to_string()),
        };
        Show::new(db).today();
    }
}
