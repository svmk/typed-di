use crate::service::service_name::ServiceName;
use std::fmt;

#[derive(Debug)]
pub struct ServiceCastError {
    service_id: ServiceName,
}

impl ServiceCastError {
    pub fn new(
        service_id: ServiceName,
    ) -> ServiceCastError {
        return ServiceCastError {
            service_id,
        }
    }
}

impl fmt::Display for ServiceCastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service `{:?}` cannot be casted", self.service_id)
    }
}

impl std::error::Error for ServiceCastError {}