use crate::service::service_name::ServiceName;
use std::fmt;
#[derive(Debug)]
pub struct ServiceNotFoundError {
    id: ServiceName,
}

impl ServiceNotFoundError {
    pub fn new(id: ServiceName) -> ServiceNotFoundError {
        return ServiceNotFoundError {
            id,
        }
    }
}

impl fmt::Display for ServiceNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Unable to find service with id `{:?}` in container", self.id)
    }
}

impl std::error::Error for ServiceNotFoundError {}