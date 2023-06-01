pub mod core;
pub mod db;
pub mod ui;

use ui::actions::traits::Action;

use crate::core::config::Config;
use crate::db::sqlite::Sqlite;
use crate::ui::cmd::command;

use crate::ui::actions::show::Show;
use crate::ui::actions::start::Start;
use crate::ui::actions::stop::Stop;
use crate::ui::actions::modify::Modify;
use crate::ui::actions::reopen::Reopen;
use crate::ui::actions::report::Report;

fn main() {
    let config = Config::new();
    let db = Sqlite::new(config);

    match command().subcommand() {
        Some(("start", sub_m)) => Start::perform(&db, &sub_m),
        Some(("stop", sub_m)) => Stop::perform(&db, &sub_m),
        Some(("modify", sub_m)) => Modify::perform(&db, &sub_m),
        Some(("reopen", sub_m)) => Reopen::perform(&db, &sub_m),
        Some(("show", sub_m)) => Show::perform(&db, &sub_m),
        Some(("report", sub_m)) => Report::perform(&db, &sub_m),
        _ => Show::new(&db).today(),
    }
}
