use clap::{Arg, ArgGroup, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Modify {}

impl<'a> Modify {
    fn desc(db: &'a dyn Db, id: &i64, desc: &String) {
        match db.change_task_desc(id, desc) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }

    fn external_id(db: &'a dyn Db, id: &i64, external_id: &String) {
        match db.change_task_external_id(id, external_id) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }

    fn project(db: &'a dyn Db, id: &i64, external_id: &String) {
        match db.change_task_project(id, external_id) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }
}

impl Action for Modify {
    const NAME: &'static str = "modify";

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();

        if let Some(desc) = sub_m.get_one::<String>("desc") {
            Self::desc(db, &id, &desc);
        }

        if let Some(external_id) = sub_m.get_one::<String>("external_id") {
            Self::external_id(db, &id, &external_id);
        }

        if let Some(project) = sub_m.get_one::<String>("project") {
            Self::project(db, &id, &project);
        }

        Show::new(db).one_task(db.task(&id).unwrap());
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME)
            .about("Modify a task's description")
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .help("Task ID")
                    .value_parser(clap::value_parser!(i64))
                    .required(true),
            )
            .arg(
                Arg::new("desc")
                    .short('d')
                    .long("desc")
                    .help("Description")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(&["project", "external_id"]),
            )
            .arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .help("Project name")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(&["desc", "external_id"]),
            )
            .arg(
                Arg::new("external_id")
                    .short('e')
                    .long("external_id")
                    .help("External ID")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(&["project", "desc"]),
            )
            .group(
                ArgGroup::new("modify")
                    .args(["desc", "external_id", "project"])
                    .required(true),
            )
    }
}
