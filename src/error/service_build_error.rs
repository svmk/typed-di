use crate::error::BuildError;
use crate::service::service_name::ServiceName;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServiceBuildError {
    service_id: ServiceName,
    error: BuildError,
}

impl ServiceBuildError {
    pub fn new(service_id: ServiceName, error: BuildError) -> ServiceBuildError {
        return ServiceBuildError {
            service_id,
            error,
        }
    }
}

impl fmt::Display for ServiceBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service `{:?}` error: `{}`", self.service_id, self.error)
    }
}

impl Error for ServiceBuildError {
    
}