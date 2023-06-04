use crate::core::config::{Config, IntegrationType};

pub mod redmine;
pub mod traits;

pub fn get_integration(config: &Config) -> impl traits::Integration {
    match config.int_type {
        IntegrationType::Redmine => redmine::Redmine::new(),
    }
}
