use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::success;
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct AddTodo {}

impl Action for AddTodo {
    const NAME: &'static str = "add-todo";

    fn subcomand() -> clap::Command {
        Command::new(Self::NAME)
            .about("Add a TODO task")
            .arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .help("Project")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                Arg::new("desc")
                    .short('d')
                    .long("desc")
                    .help("Description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            )
    }

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let desc = sub_m.get_one::<String>("desc").unwrap();
        let project = sub_m.get_one::<String>("project").unwrap();
        db.todo_add(&project, &desc).unwrap();
        success("Todo added".to_string());
        Show::new(db).today();
    }
}
