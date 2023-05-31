use clap::Parser;
use mytime::cli::{Cli, Commands, ShowRange};
use mytime::config::Config;
use mytime::start::Start;
use mytime::stop::Stop;
use mytime::show::Show;
use mytime::modify::Modify;
use mytime::reopen::Reopen;
use mytime::db::sqlite::Sqlite;

fn main() {
    let config = Config::new();
    let db = Sqlite::new(config);

    let cli = Cli::parse();
    let show = Show::new(&db);

    match &cli.command {
        Commands::Start(start_options) => {
            Start::task(&db, start_options.desc.clone());
            show.today();
        },
        Commands::Stop => {
            // Stop::active(&config);
            // show.today();
        },
        Commands::Show(show_options) => {
            match show_options.range {
                ShowRange::Today => show.today(),
                ShowRange::Week => show.week(),
                ShowRange::Month => show.month(),
            }
        },
        Commands::Modify(modify_options) => {
            // Modify::task(&config, modify_options.id, modify_options.desc.clone());
            // show.today();
        },
        Commands::Reopen(reopen_options) =>{
            // Reopen::task(&config, reopen_options.id);
            // show.today();
        }
    }
}
