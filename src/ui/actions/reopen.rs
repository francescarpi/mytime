use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct Reopen {}

impl<'a> Reopen {
    fn task(db: &'a dyn Db, id: &i64) {
        match db.active_task() {
            Ok(task) => error(format!(
                "There is an active task (task {}). It's not possible to open another one.",
                task.id
            )),
            Err(_) => match db.task(id) {
                Ok(task) => {
                    db.reopen_id(&task.id).unwrap();
                    success("Task opened again!".to_string());
                }
                Err(_) => error(format!("The task {} does not exists", id)),
            },
        };
    }
}

impl Action for Reopen {
    const NAME: &'static str = "reopen";

    fn perform<'a, 'b>(_config: &'b Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();
        Self::task(db, &id);
        Show::new(db).today();
    }

    fn subcomand() -> Command {
        Command::new(Self::NAME).about("Reopen a closed task").arg(
            Arg::new("id")
                .short('i')
                .long("id")
                .help("Task ID")
                .value_parser(clap::value_parser!(i64))
                .required(true),
        )
    }
}
