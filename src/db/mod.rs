use crate::core::config::{Config, DbType};
use sqlite::Sqlite;

pub mod sqlite;
pub mod traits;

pub fn get_db(config: &Config) -> impl traits::Db {
    match config.db_type {
        DbType::Sqlite => Sqlite::new(&config),
    }
}
