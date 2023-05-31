use clap::Parser;

pub mod ui;
pub mod core;
pub mod db;

use crate::ui::cli::{Commands, ShowRange, Cli};
use crate::core::config::Config;
use crate::db::sqlite::Sqlite;
use crate::ui::actions;

fn main() {
    let config = Config::new();
    let db = Sqlite::new(config);

    let cli = Cli::parse();
    let show = actions::show::Show::new(&db);

    match &cli.command {
        Commands::Start(start_options) => {
            actions::start::Start::task(&db, start_options.desc.clone());
            show.today();
        }
        Commands::Stop => {
            actions::stop::Stop::active(&db);
            show.today();
        }
        Commands::Show(show_options) => match show_options.range {
            ShowRange::Today => show.today(),
            ShowRange::Week => show.week(),
            ShowRange::Month => show.month(),
        },
        Commands::Modify(modify_options) => {
            actions::modify::Modify::task(&db, modify_options.id, modify_options.desc.clone());
            show.today();
        }
        Commands::Reopen(reopen_options) => {
            actions::reopen::Reopen::task(&db, reopen_options.id);
            show.today();
        }
    }
}
