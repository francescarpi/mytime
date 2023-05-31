use clap::Parser;

pub mod ui;
pub mod core;
pub mod db;

use ui::cli::{Commands, ShowRange};

fn main() {
    let config = core::config::Config::new();
    let db = db::sqlite::Sqlite::new(config);

    let cli = ui::cli::Cli::parse();
    let show = ui::show::Show::new(&db);

    match &cli.command {
        Commands::Start(start_options) => {
            ui::start::Start::task(&db, start_options.desc.clone());
            show.today();
        }
        Commands::Stop => {
            ui::stop::Stop::active(&db);
            show.today();
        }
        Commands::Show(show_options) => match show_options.range {
            ShowRange::Today => show.today(),
            ShowRange::Week => show.week(),
            ShowRange::Month => show.month(),
        },
        Commands::Modify(modify_options) => {
            ui::modify::Modify::task(&db, modify_options.id, modify_options.desc.clone());
            show.today();
        }
        Commands::Reopen(reopen_options) => {
            ui::reopen::Reopen::task(&db, reopen_options.id);
            show.today();
        }
    }
}
