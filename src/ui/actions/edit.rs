use chrono::NaiveTime;
use clap::{Arg, ArgGroup, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Edit {}

impl<'a> Edit {
    fn desc(db: &'a dyn Db, id: &i64, desc: &String) {
        db.change_task_desc(id, desc).unwrap();
    }

    fn external_id(db: &'a dyn Db, id: &i64, external_id: &String) {
        db.change_task_external_id(id, external_id).unwrap();
    }

    fn project(db: &'a dyn Db, id: &i64, project: &String) {
        db.change_task_project(id, project).unwrap();
    }

    fn start_time(db: &'a dyn Db, id: &i64, start_time: &NaiveTime) {
        db.change_task_start_time(id, start_time).unwrap();
    }

    fn end_time(db: &'a dyn Db, id: &i64, end_time: &NaiveTime) {
        db.change_task_end_time(id, end_time).unwrap();
    }

    fn validate_time(time: &str) -> Result<NaiveTime, String> {
        match NaiveTime::parse_from_str(time, "%H:%M") {
            Ok(time) => Ok(time),
            Err(_) => Err(String::from("Invalid time")),
        }
    }
}

impl Action for Edit {
    const NAME: &'static str = "edit";

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();

        match db.task(&id) {
            Ok(task) => match task.reported {
                true => {
                    error("The task was reported. It cannot be editted".to_string());
                    return;
                }
                false => (),
            },
            Err(_) => {
                error("The task does not exists".to_string());
                return;
            },
        }

        if let Some(desc) = sub_m.get_one::<String>("desc") {
            Self::desc(db, &id, &desc);
        }

        if let Some(external_id) = sub_m.get_one::<String>("external_id") {
            Self::external_id(db, &id, &external_id);
        }

        if let Some(project) = sub_m.get_one::<String>("project") {
            Self::project(db, &id, &project);
        }

        if let Some(start_time) = sub_m.get_one::<NaiveTime>("start_time") {
            Self::start_time(db, &id, &start_time);
        }

        if let Some(end_time) = sub_m.get_one::<NaiveTime>("end_time") {
            Self::end_time(db, &id, &end_time);
        }

        success("Task updated!".to_string());
        Show::new(db).one_task(db.task(&id).unwrap());
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME)
            .about("Edit a task")
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
                    .conflicts_with_all(&["project", "external_id", "start_time", "end_time"]),
            )
            .arg(
                Arg::new("project")
                    .short('p')
                    .long("project")
                    .help("Project name")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(&["desc", "external_id", "start_time", "end_time"]),
            )
            .arg(
                Arg::new("external_id")
                    .short('e')
                    .long("external_id")
                    .help("External ID")
                    .value_parser(clap::value_parser!(String))
                    .conflicts_with_all(&["project", "desc", "start_time", "end_time"]),
            )
            .arg(
                Arg::new("start_time")
                    .short('s')
                    .long("start_time")
                    .help("Format: HH:MM")
                    .value_parser(Self::validate_time)
                    .conflicts_with_all(&["project", "desc", "external_id", "end_time"]),
            )
            .arg(
                Arg::new("end_time")
                    .short('n')
                    .long("end_time")
                    .help("Format: HH:MM")
                    .value_parser(Self::validate_time)
                    .conflicts_with_all(&["project", "desc", "external_id", "start_time"]),
            )
            .group(
                ArgGroup::new("edit")
                    .args(["desc", "external_id", "project", "start_time", "end_time"])
                    .required(true),
            )
    }
}
