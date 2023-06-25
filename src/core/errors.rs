use std::fmt;

#[derive(Debug)]
pub enum Error {
    TaskDoesNotExist,
    ExistActiveTask,
    TaskCannotBeenReported(String),
    MissingIntegrationParams,
    TodoDoesNotExist,
    TaskDoesNotHaveEndDate,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::TaskDoesNotExist => write!(f, "Task does not exist"),
            Error::ExistActiveTask => write!(f, "Exists an active task"),
            Error::TaskCannotBeenReported(reason) => {
                write!(f, "Task cannot been reported: {}", reason)
            }
            Error::MissingIntegrationParams => write!(f, "Missing integration configuration"),
            Error::TodoDoesNotExist => write!(f, "Todo does not exist"),
            Error::TaskDoesNotHaveEndDate => write!(f, "Task does not have end date"),
        }
    }
}
