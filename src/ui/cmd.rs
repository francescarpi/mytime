use chrono::NaiveDate;
use clap::{Arg, ArgMatches, Command};

pub fn command() -> ArgMatches {
    Command::new("mytime")
        .author("Francesc Arpí Roca")
        .version("0.1.") // get from cargo
        .about("Program to tracker your working time")
        .subcommand(
            Command::new("start").about("Start a new task").arg(
                Arg::new("desc")
                    .short('d')
                    .help("Description")
                    .required(true)
                    .value_parser(clap::value_parser!(String)),
            ),
        )
        .subcommand(Command::new("stop").about("Stop de active task"))
        .subcommand(
            Command::new("show")
                .about("Display the tasks table")
                .arg(
                    Arg::new("period")
                        .short('p')
                        .conflicts_with_all(&["relative", "date"])
                        .value_parser(["today", "week", "month"]),
                )
                .arg(
                    Arg::new("relative")
                        .short('r')
                        .conflicts_with_all(&["period", "date"])
                        .help("1 == -1 == yesterday")
                        .value_parser(clap::value_parser!(i64).range(0..=7)),
                )
                .arg(
                    Arg::new("date")
                        .short('d')
                        .conflicts_with_all(&["relative", "period"])
                        .value_parser(validate_date)
                        .help("Format: YYYY-MM-DD"),
                ),
        )
        .subcommand(
            Command::new("modify")
                .about("Modify a task's description")
                .arg(
                    Arg::new("id")
                        .short('i')
                        .help("Task ID")
                        .value_parser(clap::value_parser!(i64))
                        .required(true),
                )
                .arg(
                    Arg::new("desc")
                        .short('d')
                        .help("Description")
                        .value_parser(clap::value_parser!(String))
                        .required(true),
                ),
        )
        .subcommand(
            Command::new("reopen").about("Reopen a closed task").arg(
                Arg::new("id")
                    .short('i')
                    .help("Task ID")
                    .value_parser(clap::value_parser!(i64))
                    .required(true),
            ),
        )
        .get_matches()
}

fn validate_date(date: &str) -> Result<NaiveDate, String> {
    match NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(String::from("Invalid date")),
    }
}
