use clap::{Arg, ArgMatches, Command};

use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Start {}

impl Action for Start {
    fn perform<'a>(db: &'a dyn Db, sub_m: &ArgMatches) {
        let desc = sub_m.get_one::<String>("desc").unwrap();

        match db.add_task(desc.clone()) {
            Ok(_) => success("Task added successfully!".to_string()),
            Err(_) => {
                error("There is another active task. You have to stop it before.".to_string())
            }
        }

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new("start").about("Start a new task").arg(
            Arg::new("desc")
                .short('d')
                .help("Description")
                .required(true)
                .value_parser(clap::value_parser!(String)),
        )
    }
}
