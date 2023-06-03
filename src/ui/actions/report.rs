use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Report {}

impl Action for Report {
    fn perform<'a, 'b>(_config: &'b Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();

        match db.report_task(&id) {
            Ok(_) => success("Task mark as reported!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new("report")
            .about("Marks manually a task as a reported (toggle)")
            .arg(
                Arg::new("id")
                    .short('i')
                    .help("Task ID")
                    .value_parser(clap::value_parser!(i64))
                    .required(true),
            )
    }
}
