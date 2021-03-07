use crate::argument::argument_name::ArgumentName;
use std::fmt;
#[derive(Debug)]
pub struct ArgumentNotFoundError {
    id: ArgumentName,
}

impl ArgumentNotFoundError {
    pub fn new(id: ArgumentName) -> ArgumentNotFoundError {
        return ArgumentNotFoundError {
            id,
        }
    }
}

impl fmt::Display for ArgumentNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find argument with id `{:?}` in container", self.id)
    }
}

impl std::error::Error for ArgumentNotFoundError {}