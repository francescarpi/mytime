use clap::ArgMatches;

use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::actions::traits::Action;

pub struct Report {}

impl Action for Report {
    fn perform<'a>(db: &'a dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();

        match db.report_task(id.clone()) {
            Ok(_) => success("Task mark as reported!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };

        Show::new(db).today();
    }
}
