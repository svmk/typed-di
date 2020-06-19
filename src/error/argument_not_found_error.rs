use std::fmt;
#[derive(Debug)]
pub struct ArgumentNotFoundError {
    id: String,
}

impl ArgumentNotFoundError {
    pub fn new(id: String) -> ArgumentNotFoundError {
        return ArgumentNotFoundError {
            id,
        }
    }
}

impl fmt::Display for ArgumentNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find argument with id `{}` in container", self.id)
    }
}

impl std::error::Error for ArgumentNotFoundError {}