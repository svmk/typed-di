use crate::service::service_name::ServiceName;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServiceBuilderNotFound {
    service_name: ServiceName,
}

impl ServiceBuilderNotFound {
    pub fn new(service_name: ServiceName) -> ServiceBuilderNotFound {
        return ServiceBuilderNotFound {
            service_name,
        }
    }
}

impl fmt::Display for ServiceBuilderNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service builder `{:?}` not found", self.service_name)
    }
}

impl Error for ServiceBuilderNotFound {
    
}