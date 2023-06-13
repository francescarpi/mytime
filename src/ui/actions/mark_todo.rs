use clap::{Arg, ArgMatches, Command};

use crate::core::config::Config;
use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::traits::Action;

pub struct MarkTodo {}

impl Action for MarkTodo {
    const NAME: &'static str = "mark_todo";

    fn subcomand() -> clap::Command {
        Command::new(Self::NAME).about("Mark a TODO as done").arg(
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

        match db.todo_mark_as_done(&id) {
            Ok(_todo) => success("Done!".to_string()),
            Err(_) => error("TODO does not exist".to_string()),
        };

        Show::new(db).today();
    }
}
