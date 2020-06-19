use std::fmt;
#[derive(Debug)]
pub struct ServiceNotFoundError {
    id: String,
}

impl ServiceNotFoundError {
    pub fn new(id: String) -> ServiceNotFoundError {
        return ServiceNotFoundError {
            id,
        }
    }
}

impl fmt::Display for ServiceNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find service with id `{}` in container", self.id)
    }
}

impl std::error::Error for ServiceNotFoundError {}