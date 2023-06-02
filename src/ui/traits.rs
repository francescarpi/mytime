use crate::core::config::Config;
use crate::db::traits::Db;
use clap::{ArgMatches, Command};

pub trait Action {
    fn perform<'a, 'b>(config: &'a Config, db: &'b dyn Db, sub_m: &ArgMatches);
    fn subcomand() -> Command;
}
