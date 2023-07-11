use clap::{Arg, ArgAction, ArgGroup, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Reopen {}

impl<'a> Reopen {
    fn reopen_with_id(db: &'a dyn Db, id: &i64) {
        match db.task(id) {
            Ok(task) => {
                db.reopen_id(&task.id).unwrap();
                success("Task opened again!".to_string());
            }
            Err(_) => error(format!("The task {} does not exists", id)),
        };
    }

    fn reopen_last(db: &'a dyn Db) {
        match db.last_task() {
            Ok(task) => {
                db.reopen_id(&task.id).unwrap();
                success("Task opened again!".to_string());
            }
            Err(_) => error(format!("There are not tasks")),
        };
    }
}

impl Action for Reopen {
    const NAME: &'static str = "reopen";

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        match db.active_task() {
            Ok(task) => error(format!(
                "There is an active task (task {}). It's not possible to open another one.",
                task.id
            )),
            Err(_) => match sub_m.get_one::<i64>("id") {
                Some(id) => Self::reopen_with_id(db, &id),
                None => Self::reopen_last(db),
            },
        };

        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME)
            .about("Reopen a closed task")
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .help("Task ID")
                    .value_parser(clap::value_parser!(i64))
                    .conflicts_with("last"),
            )
            .arg(
                Arg::new("last")
                    .short('l')
                    .long("last")
                    .help("Last task")
                    .action(ArgAction::SetTrue)
                    .conflicts_with("id"),
            )
            .group(ArgGroup::new("reopen").args(["id", "last"]).required(true))
    }
}
