use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct TodoToTask {}

impl Action for TodoToTask {
    const NAME: &'static str = "todo-to-task";

    fn subcomand() -> clap::Command {
        Command::new(Self::NAME)
            .about("Pass one TODO to a TASK")
            .arg(
                Arg::new("id")
                    .short('i')
                    .long("id")
                    .help("Todo ID")
                    .required(true)
                    .value_parser(clap::value_parser!(i64)),
            )
    }

    fn perform<'a, 'b>(_config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();

        match db.todo_detail(&id) {
            Ok(todo) => match db.active_task() {
                Ok(_task) => error(
                    "There are an active task. Is not to posible create a new task from the TODO."
                        .to_string(),
                ),
                Err(_) => {
                    db.add_task(&todo.project, &todo.desc, &None).unwrap();
                    db.todo_mark_as_done(&todo.id).unwrap();
                    success("Done!".to_string());
                    Show::new(db).today();
                }
            },
            Err(_) => error("The task does not exists".to_string()),
        }
    }
}
