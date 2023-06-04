use crate::core::errors::Error;
use crate::core::task::Task;
use crate::core::config::Config;

pub trait Integration {
    fn report_task<'a>(&self, config: &'a Config, task: &Task) -> Result<(), Error>;
}
