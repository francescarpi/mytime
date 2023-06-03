use std::fmt;

#[derive(Debug)]
pub enum Error {
    TaskDoesNotExist,
    ExistActiveTask,
    TaskCannotBeenReported(String),
    MissingIntegrationParams,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TaskDoesNotExist => write!(f, "Task does not exist"),
            Error::ExistActiveTask => write!(f, "Exists an active task"),
            Error::TaskCannotBeenReported(reason) => write!(f, "Task cannot be reported: {}", reason),
            Error::MissingIntegrationParams => write!(f, "Missing integration configuration")
        }
    }
}
