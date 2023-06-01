use clap::ArgMatches;

use crate::core::utils::display::{error, success};
use crate::db::traits::Db;
use crate::ui::actions::show::Show;
use crate::ui::actions::traits::Action;

pub struct Modify {}

impl<'a> Modify {
    fn desc(db: &'a dyn Db, id: i64, desc: String) {
        match db.change_task_desc(id, desc) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }

    fn external_id(db: &'a dyn Db, id: i64, external_id: String) {
        match db.change_task_external_id(id, external_id) {
            Ok(_) => success("Task updated!".to_string()),
            Err(_) => error("There is not any task with this ID!".to_string()),
        };
    }
}

impl Action for Modify {
    fn perform<'a>(db: &'a dyn Db, sub_m: &ArgMatches) {
        let id = sub_m.get_one::<i64>("id").unwrap();
        if let Some(desc) = sub_m.get_one::<String>("desc") {
            Self::desc(db, id.clone(), desc.clone());
        }
        if let Some(external_id) = sub_m.get_one::<String>("external_id") {
            Self::external_id(db, id.clone(), external_id.clone());
        }
        Show::new(db).today();
    }
}
