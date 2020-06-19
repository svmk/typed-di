use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct ServiceBuildError {
    service_id: String,
    error: Box<dyn Error + Send + Sync>,
}

impl ServiceBuildError {
    pub fn new(service_id: String, error: impl Error + Send + Sync + 'static) -> ServiceBuildError {
        return ServiceBuildError {
            service_id,
            error: Box::new(error),
        }
    }
}

impl fmt::Display for ServiceBuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Service `{}` error: `{}`", self.service_id, self.error)
    }
}

impl Error for ServiceBuildError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        return Some(self.error.as_ref());
    }
}