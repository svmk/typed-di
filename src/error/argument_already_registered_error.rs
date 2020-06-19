use std::fmt;
#[derive(Debug)]
pub struct ArgumentAlreadyRegisteredError {
    id: String,
}

impl ArgumentAlreadyRegisteredError {
    pub fn new(id: String) -> ArgumentAlreadyRegisteredError {
        return ArgumentAlreadyRegisteredError {
            id,
        }
    }
}

impl fmt::Display for ArgumentAlreadyRegisteredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to insert argument with id `{}`", self.id)
    }
}

impl std::error::Error for ArgumentAlreadyRegisteredError {}