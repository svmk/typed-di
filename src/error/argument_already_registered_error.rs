use crate::argument::argument_name::ArgumentName;
use std::fmt;
#[derive(Debug)]
pub struct ArgumentAlreadyRegisteredError {
    id: ArgumentName,
}

impl ArgumentAlreadyRegisteredError {
    pub fn new(id: ArgumentName) -> ArgumentAlreadyRegisteredError {
        return ArgumentAlreadyRegisteredError {
            id,
        }
    }
}

impl fmt::Display for ArgumentAlreadyRegisteredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to insert argument with id `{:?}`", self.id)
    }
}

impl std::error::Error for ArgumentAlreadyRegisteredError {}