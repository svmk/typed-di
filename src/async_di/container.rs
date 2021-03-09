use crate::error::Error;
use crate::service::service_name::ServiceName;
use crate::argument::argument_id::ArgumentId;
use crate::argument::argument_instance::ArgumentInstance;
use crate::argument::argument_name::ArgumentName;
use crate::async_di::service_builder::ServiceBuilder;
use crate::async_di::resolver::Resolver;
use crate::service::service::Service;
use crate::service::service_id::ServiceId;
use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;

#[cfg(test)]
mod container_test;

#[derive(Debug)]
struct ContainerPayload {
    arguments: HashMap<ArgumentName, ArgumentInstance>,
    services: HashMap<ServiceName, ServiceBuilder>,
}

#[derive(Debug)]
pub struct Container {
    payload: Arc<ContainerPayload>,
}

impl Container {
    pub fn new(
        arguments: HashMap<ArgumentName, ArgumentInstance>,
        services: HashMap<ServiceName, ServiceBuilder>,
    ) -> Container {
        let payload = ContainerPayload {
            arguments,
            services,
        };
        let payload = Arc::new(payload);
        return Container {
            payload,
        }
    }

    pub (crate) fn container_clone(&self) -> Container {
        return Container {
            payload: self.payload.clone(),
        }
    }
    
    pub fn get_argument<T>(&self, id: ArgumentId<T>) -> Result<&T, Error>
        where
            T: Any + 'static,
    {
        if let Some(argument) = self.payload.arguments.get(id.get_name()) {
            return argument.downcast_ref();
        }
        return Err(Error::argument_not_found_error(id.get_name().clone()));
    }

    fn get_service_builder(&self, id: &ServiceName) -> Result<&ServiceBuilder, Error>
    {
        if let Some(service) = self.payload.services.get(id) {
            return Ok(service);
        }
        return Err(Error::service_not_found_error(id.clone()));
    }

    pub async fn get_service<T>(&self, id: ServiceId<T>) -> Result<Service<T>, Error>
        where
            T: Any + Send + 'static,
    {
        let service_builder = self.get_service_builder(id.get_name())?;
        let resolver = Resolver::new(self.container_clone());
        let service = service_builder.resolve(resolver).await?;
        return Ok(service);
    }
}