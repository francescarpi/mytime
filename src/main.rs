pub mod core;
pub mod db;
pub mod integrations;
pub mod ui;

use clap::{crate_authors, crate_name, crate_version, ColorChoice, Command};
use ui::traits::Action;

use crate::core::config::Config;
use crate::db::get_db;

use crate::ui::actions::{
    add_todo::AddTodo, mark_todo::MarkTodo, edit::Edit, reopen::Reopen, report::Report,
    send::Send, show::Show, start::Start, stop::Stop, todo_to_task::TodoToTask,
};

fn main() {
    let config = Config::new();
    let db = get_db(&config);

    let matches = Command::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(help(&config))
        .color(ColorChoice::Always)
        .subcommand(Start::subcomand())
        .subcommand(Stop::subcomand())
        .subcommand(Show::subcomand())
        .subcommand(Edit::subcomand())
        .subcommand(Reopen::subcomand())
        .subcommand(Report::subcomand())
        .subcommand(Send::subcomand())
        .subcommand(AddTodo::subcomand())
        .subcommand(MarkTodo::subcomand())
        .subcommand(TodoToTask::subcomand())
        .get_matches();

    match matches.subcommand() {
        Some((Start::NAME, sub_m)) => Start::perform(&config, &db, &sub_m),
        Some((Stop::NAME, sub_m)) => Stop::perform(&config, &db, &sub_m),
        Some((Edit::NAME, sub_m)) => Edit::perform(&config, &db, &sub_m),
        Some((Reopen::NAME, sub_m)) => Reopen::perform(&config, &db, &sub_m),
        Some((Show::NAME, sub_m)) => Show::perform(&config, &db, &sub_m),
        Some((Report::NAME, sub_m)) => Report::perform(&config, &db, &sub_m),
        Some((Send::NAME, sub_m)) => Send::perform(&config, &db, &sub_m),
        Some((AddTodo::NAME, sub_m)) => AddTodo::perform(&config, &db, &sub_m),
        Some((MarkTodo::NAME, sub_m)) => MarkTodo::perform(&config, &db, &sub_m),
        Some((TodoToTask::NAME, sub_m)) => TodoToTask::perform(&config, &db, &sub_m),
        _ => Show::new(&db).today(),
    }
}

fn help(config: &Config) -> String {
    let desc = env!("CARGO_PKG_DESCRIPTION");
    format!(
        "{}\n\nConfig file present: {}\nWorking path: {}\nDatabase: {}\nIntegration: {}",
        desc,
        config.ini_file_present,
        config.app_share_path.to_string_lossy(),
        config.db_type,
        config.int_type
    )
}
