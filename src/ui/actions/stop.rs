use clap::{ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::core::utils::formatters::format_seconds;
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Stop {}

impl Action for Stop {
    const NAME: &'static str = "stop";

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, _sub_m: &ArgMatches) {
        match db.active_task() {
            Ok(task) => {
                let task = db.stop_task(&task.id).unwrap();

                println!("\nTask ID: {}", task.id);
                println!("Description: {}", task.desc);
                println!("Duration: {}", format_seconds(&task.duration()));

                success("Stopped!".to_string());
            }
            Err(_) => error("There is not any active task!".to_string()),
        };
        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME).about("Stop de active task")
    }
}
