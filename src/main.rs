pub mod core;
pub mod db;
pub mod ui;

use crate::core::config::Config;
use crate::db::sqlite::Sqlite;
use crate::ui::actions;
use crate::ui::cmd::command;

fn main() {
    let config = Config::new();
    let db = Sqlite::new(config);

    let show = actions::show::Show::new(&db);
    let matches = command();

    match matches.subcommand() {
        Some(("start", sub_m)) => {
            let desc = sub_m.get_one::<String>("desc").unwrap();
            actions::start::Start::task(&db, desc.clone());
            show.today();
        }
        Some(("stop", _)) => {
            actions::stop::Stop::active(&db);
            show.today();
        }
        Some(("modify", sub_m)) => {
            let desc = sub_m.get_one::<String>("desc").unwrap();
            let id = sub_m.get_one::<i64>("id").unwrap();
            actions::modify::Modify::task(&db, id.clone(), desc.clone());
            show.today();
        }
        Some(("reopen", sub_m)) => {
            let id = sub_m.get_one::<i64>("id").unwrap();
            actions::reopen::Reopen::task(&db, id.clone());
            show.today();
        }
        Some(("show", sub_m)) => {
            if let Some(period) = sub_m.get_one::<String>("period") {
                match period.as_str() {
                    "today" => show.today(),
                    "week" => show.week(),
                    "month" => show.month(),
                    _ => show.today(),
                };
            } else if let Some(relative) = sub_m.get_one::<u8>("relative") {
                dbg!(relative);
            } else if let Some(date) = sub_m.get_one::<String>("date") {
                dbg!(date);
            } else {
                show.today();
            }
        }
        _ => {
            show.today();
        }
    }
}
