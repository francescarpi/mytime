use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Report {}

impl Action for Report {
    const NAME: &'static str = "report";

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();

        match db.report_task(&id) {
            Ok(_) => success("Task mark as reported!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };

        Show::new(db).one_task(db.task(&id).unwrap());
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME)
            .about("Mark manually a task as a reported")
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .help("Task ID")
                    .value_parser(clap::value_parser!(i64))
                    .required(true),
            )
    }
}
