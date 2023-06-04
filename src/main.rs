pub mod core;
pub mod db;
pub mod integrations;
pub mod ui;

use clap::{crate_authors, crate_name, crate_version, Command};
use ui::traits::Action;

use crate::core::config::Config;
use crate::db::get_db;

use crate::ui::actions::{
    modify::Modify, reopen::Reopen, report::Report, send::Send, show::Show, start::Start,
    stop::Stop,
};

fn main() {
    let config = Config::new();
    let db = get_db(&config);

    let matches = Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(help(&config))
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

fn help(config: &Config) -> String {
    let desc = env!("CARGO_PKG_DESCRIPTION");
    format!(
        "{}\n\nWorking path: {}\nDatabase: {}\nIntegration: {}",
        desc,
        config.app_share_path.to_string_lossy(),
        config.db_type,
        config.int_type
    )
}
