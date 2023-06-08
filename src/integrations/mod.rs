use crate::core::config::{Config, IntegrationType};

pub mod redmine;
pub mod traits;

#[derive(Debug, Ord, PartialEq, PartialOrd, Eq)]
pub struct IntegrationTask {
    pub external_id: String,
    pub duration: i64,
    pub desc: String,
    pub start: String,
    pub ids_used: Vec<i64>,
}

pub fn get_integration(config: &Config) -> impl traits::Integration {
    match config.int_type {
        IntegrationType::Redmine => redmine::Redmine::new(),
    }
}
