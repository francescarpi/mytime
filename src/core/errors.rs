#[derive(Debug)]
pub enum Error {
    TaskDoesNotExist,
    ExistActiveTask,
    TaskCannotBeenReported,
    MissingIntegrationParams,
}

