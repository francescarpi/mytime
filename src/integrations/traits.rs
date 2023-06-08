use crate::core::errors::Error;
use crate::core::config::Config;
use crate::integrations::IntegrationTask;

pub trait Integration {
    fn report_task<'a>(&self, config: &'a Config, task: &IntegrationTask) -> Result<(), Error>;
}
