use std::fmt;
use std::error::Error as StdError;
use crate::service::service_name::ServiceName;
use crate::argument::argument_name::ArgumentName;
mod argument_not_found_error;
pub use self::argument_not_found_error::ArgumentNotFoundError;
mod service_not_found_error;
pub use self::service_not_found_error::ServiceNotFoundError;
mod service_cast_error;
pub use self::service_cast_error::ServiceCastError;
mod argument_already_registered_error;
pub use self::argument_already_registered_error::ArgumentAlreadyRegisteredError;
mod service_already_registered_error;
pub use self::service_already_registered_error::ServiceAlreadyRegisteredError;
mod argument_downcast_error;
pub use self::argument_downcast_error::ArgumentDowncastError;
mod service_build_error;
pub use self::service_build_error::ServiceBuildError;
mod build_error;
pub use self::build_error::BuildError;
mod build_error_cast;
pub use self::build_error_cast::BuildErrorCast;
mod service_builder_not_found;
pub use self::service_builder_not_found::ServiceBuilderNotFound;

#[derive(Debug)]
pub enum Error {
    ArgumentNotFound(ArgumentNotFoundError),
    ServiceNotFound(ServiceNotFoundError),
    ServiceCastError(ServiceCastError),
    RegisterArgument(ArgumentAlreadyRegisteredError),
    ServiceAlreadyRegistered(ServiceAlreadyRegisteredError),
    ArgumentDowncastError(ArgumentDowncastError),
    ServiceBuildError(ServiceBuildError),
    ServiceBuilderNotFound(ServiceBuilderNotFound),
}

impl Error {
    pub fn argument_not_found_error(id: ArgumentName) -> Error {
        return Error::ArgumentNotFound(ArgumentNotFoundError::new(id));
    }

    pub fn service_not_found_error(id: ServiceName) -> Error {
        return Error::ServiceNotFound(ServiceNotFoundError::new(id));
    }

    pub fn service_cast_error(
        service_name: ServiceName,
    ) -> Error {
        return Error::ServiceCastError(ServiceCastError::new(service_name));
    }

    pub fn argument_already_registered(id: ArgumentName) -> Error {
        return Error::RegisterArgument(ArgumentAlreadyRegisteredError::new(id));
    }

    pub fn service_already_registered(id: ServiceName) -> Error {
        return Error::ServiceAlreadyRegistered(ServiceAlreadyRegisteredError::new(id));
    }

    pub fn argument_downcast(id: ArgumentName, argument_type: String) -> Error {
        return Error::ArgumentDowncastError(ArgumentDowncastError::new(id, argument_type));
    }

    pub fn service_build(service_id: ServiceName, error: BuildError) -> Error {
        return Error::ServiceBuildError(ServiceBuildError::new(service_id, error));
    }

    pub fn service_builder_not_found(service_id: ServiceName) -> Error {
        return Error::ServiceBuilderNotFound(ServiceBuilderNotFound::new(service_id));
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.source() {
            Some(error) => {
                write!(f, "{}", error)
            },
            None => {
                write!(f, "Unknown error")
            },
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::ArgumentNotFound(ref error) => {
                return Some(error);
            },
            Error::ServiceNotFound(ref error) => {
                return Some(error);
            },
            Error::ServiceCastError(ref error) => {
                return Some(error);
            },
            Error::RegisterArgument(ref error) => {
                return Some(error);
            },
            Error::ServiceAlreadyRegistered(ref error) => {
                return Some(error);
            },
            Error::ArgumentDowncastError(ref error) => {
                return Some(error);
            },
            Error::ServiceBuildError(ref error) => {
                return Some(error);
            },
            Error::ServiceBuilderNotFound(ref error) => {
                return Some(error);
            },
        }
    }
}

impl From<ArgumentNotFoundError> for Error {
    fn from(error: ArgumentNotFoundError) -> Self {
        return Error::ArgumentNotFound(error);
    }
}

impl From<ServiceNotFoundError> for Error {
    fn from(error: ServiceNotFoundError) -> Self {
        return Error::ServiceNotFound(error);
    }
}

impl From<ServiceCastError> for Error {
    fn from(error: ServiceCastError) -> Self {
        return Error::ServiceCastError(error);
    }
}

impl From<ArgumentAlreadyRegisteredError> for Error {
    fn from(error: ArgumentAlreadyRegisteredError) -> Self {
        return Error::RegisterArgument(error);
    }
}

impl From<ServiceAlreadyRegisteredError> for Error {
    fn from(error: ServiceAlreadyRegisteredError) -> Self {
        return Error::ServiceAlreadyRegistered(error);
    }
}

impl From<ArgumentDowncastError> for Error {
    fn from(error: ArgumentDowncastError) -> Self {
        return Error::ArgumentDowncastError(error);
    }
}

impl From<ServiceBuildError> for Error {
    fn from(error: ServiceBuildError) -> Self {
        return Error::ServiceBuildError(error);
    }
}

impl From<ServiceBuilderNotFound> for Error {
    fn from(error: ServiceBuilderNotFound) -> Self {
        return Error::ServiceBuilderNotFound(error);
    }
}