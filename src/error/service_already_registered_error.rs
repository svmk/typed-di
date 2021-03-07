use crate::service::service_name::ServiceName;
use std::fmt;
#[derive(Debug)]
pub struct ServiceAlreadyRegisteredError {
    id: ServiceName,
}

impl ServiceAlreadyRegisteredError {
    pub fn new(id: ServiceName) -> ServiceAlreadyRegisteredError {
        return ServiceAlreadyRegisteredError {
            id,
        }
    }
}

impl fmt::Display for ServiceAlreadyRegisteredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service with id `{:?}` already registered", self.id)
    }
}

impl std::error::Error for ServiceAlreadyRegisteredError {}