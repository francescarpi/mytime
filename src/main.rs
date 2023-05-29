use clap::Parser;
use mytime::cli::{Cli, Commands, ShowRange};
use mytime::config::Config;
use mytime::start::Start;
use mytime::stop::Stop;
use mytime::show::Show;

fn main() {
    let config = Config::init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Start(start_options) => {
            // TODO: Comprobar si existeix una tasca prèvia, per què s'hauria de tancar!
            // TODO: Mostrar la taula
            Start::task(config, start_options.desc.clone());
        },
        Commands::Stop => {
            Stop::active(config);
        },
        Commands::Show(show_options) => {
            let show = Show::new(config);
            match show_options.range {
                ShowRange::Today => show.today(),
                ShowRange::Week => show.week(),
                ShowRange::Month => show.month(),
            }
        },
    }
}
