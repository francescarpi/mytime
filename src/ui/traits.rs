use clap::{ArgMatches, Command};
use crate::db::traits::Db;

pub trait Action {
    fn perform<'a>(db: &'a dyn Db, sub_m: &ArgMatches);
    fn subcomand() -> Command;
}
