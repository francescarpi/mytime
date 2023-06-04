use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Start {}

impl Action for Start {
    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let desc = sub_m.get_one::<String>("desc").unwrap();
        let project = sub_m.get_one::<String>("project").unwrap();
        let external_id = sub_m.get_one::<String>("external_id").map(|value| value.clone());

        match db.add_task(&project, &desc, &external_id) {
            Ok(_) => success("Task added successfully!".to_string()),
            Err(_) => {
                error("There is another active task. You have to stop it before.".to_string())
            }
        }

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new("start")
            .about("Start a new task")
            .arg(
                Arg::new("desc")
                    .short('d')
                    .long("desc")
                    .help("Description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .help("Project name")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                Arg::new("external_id")
                    .short('e')
                    .long("external_id")
                    .help("External ID")
                    .default_value(None)
                    .value_parser(clap::value_parser!(String)),
            )
    }
}
