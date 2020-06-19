use crate::error::Error;
use crate::argument_id::ArgumentId;
use crate::argument_instance::ArgumentInstance;
use crate::service_builder::ServiceBuilder;
use std::collections::HashMap;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct DiContainer {
    arguments: HashMap<&'static str, ArgumentInstance>,
    services: HashMap<&'static str, ServiceBuilder>,
}

impl DiContainer {
    pub fn new(
        arguments: HashMap<&'static str, ArgumentInstance>,
        services: HashMap<&'static str, ServiceBuilder>,
    ) -> DiContainer {
        return DiContainer {
            arguments,
            services,
        }
    }
    
    pub fn get_argument<T>(&self, id: ArgumentId<T>) -> Result<&T, Error>
        where
            T: Any + 'static,
    {
        if let Some(argument) = self.arguments.get(id.get_id()) {
            return argument.downcast_ref();
        }
        return Err(Error::argument_not_found_error(id.get_id().to_string()));
    }

    pub fn get_service_builder(&self, id: &str) -> Result<&ServiceBuilder, Error>
    {
        if let Some(service) = self.services.get(id) {
            return Ok(service);
        }
        return Err(Error::service_not_found_error(id.to_string()));
    }
}