use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Start a new task
    Start(StartOptions),

    /// Stop active task
    Stop,

    /// Display the status table
    Show(ShowOptions),
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ShowRange {
    Today,
    Week,
    Month,
}

#[derive(Args, Debug)]
pub struct StartOptions {
    #[arg(short, long)]
    pub desc: String,
}

#[derive(Args, Debug)]
pub struct ShowOptions {
    #[arg(short, long, value_enum, default_value_t=ShowRange::Today)]
    pub range: ShowRange,
}
