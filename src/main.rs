pub mod core;
pub mod db;
pub mod ui;

use ui::traits::Action;
use clap::Command;

use crate::core::config::Config;
use crate::db::sqlite::Sqlite;

use crate::ui::actions::modify::Modify;
use crate::ui::actions::reopen::Reopen;
use crate::ui::actions::report::Report;
use crate::ui::actions::show::Show;
use crate::ui::actions::start::Start;
use crate::ui::actions::stop::Stop;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let config = Config::new();
    let db = Sqlite::new(config);

    let matches = Command::new("mytime")
        .author(AUTHORS)
        .version(VERSION) // get from cargo
        .about("Program to tracker your working time")
        .subcommand(Start::subcomand())
        .subcommand(Stop::subcomand())
        .subcommand(Show::subcomand())
        .subcommand(Modify::subcomand())
        .subcommand(Reopen::subcomand())
        .subcommand(Report::subcomand())
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_m)) => Start::perform(&db, &sub_m),
        Some(("stop", sub_m)) => Stop::perform(&db, &sub_m),
        Some(("modify", sub_m)) => Modify::perform(&db, &sub_m),
        Some(("reopen", sub_m)) => Reopen::perform(&db, &sub_m),
        Some(("show", sub_m)) => Show::perform(&db, &sub_m),
        Some(("report", sub_m)) => Report::perform(&db, &sub_m),
        _ => Show::new(&db).today(),
    }
}
