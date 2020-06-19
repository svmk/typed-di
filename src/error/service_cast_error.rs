use std::fmt;

#[derive(Debug)]
pub struct ServiceCastError {
    service_id: String,
    type_name: String,
    service_type: String,
}

impl ServiceCastError {
    pub fn new(
        service_id: String,
        type_name: String,
        service_type: String,
    ) -> ServiceCastError {
        return ServiceCastError {
            service_id,
            type_name,
            service_type,
        }
    }
}

impl fmt::Display for ServiceCastError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service `{}` cannot be casted as `{}`: actual type `{}`", self.service_id, self.type_name, self.service_type)
    }
}

impl std::error::Error for ServiceCastError {}