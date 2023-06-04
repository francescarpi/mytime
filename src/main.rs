pub mod core;
pub mod db;
pub mod integrations;
pub mod ui;

use clap::Command;
use ui::traits::Action;

use crate::core::config::Config;
use crate::db::get_db;

use crate::ui::actions::{
    modify::Modify, reopen::Reopen, report::Report, send::Send, show::Show, start::Start,
    stop::Stop,
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() {
    let config = Config::new();
    let db = get_db(&config);

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
        .subcommand(Send::subcomand())
        .get_matches();

    match matches.subcommand() {
        Some(("start", sub_m)) => Start::perform(&config, &db, &sub_m),
        Some(("stop", sub_m)) => Stop::perform(&config, &db, &sub_m),
        Some(("modify", sub_m)) => Modify::perform(&config, &db, &sub_m),
        Some(("reopen", sub_m)) => Reopen::perform(&config, &db, &sub_m),
        Some(("show", sub_m)) => Show::perform(&config, &db, &sub_m),
        Some(("report", sub_m)) => Report::perform(&config, &db, &sub_m),
        Some(("send", sub_m)) => Send::perform(&config, &db, &sub_m),
        _ => Show::new(&db).today(),
    }
}
