use crate::error::Error;
use crate::argument_instance::ArgumentInstance;
use crate::service_id::ServiceId;
use crate::argument_id::ArgumentId;
use crate::service_builder::ServiceBuilder;
use crate::di_container::DiContainer;
use std::collections::HashMap;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct ContainerDeclaration {
    arguments: HashMap<&'static str, ArgumentInstance>,
    services: HashMap<&'static str, ServiceBuilder>,
}

impl ContainerDeclaration {
    pub fn new() -> ContainerDeclaration {
        return ContainerDeclaration {
            arguments: HashMap::new(),
            services: HashMap::new(),
        }
    }

    pub fn add_argument<T>(&mut self, id: ArgumentId<T>, argument: T) -> Result<(), Error>
        where
            T: 'static,
            T: Any,
            T: Send,
            T: Sync,
    {
        if self.arguments.contains_key(id.get_id()) {
            return Err(Error::argument_already_registered(id.get_id().to_string()));
        }
        let _ = self.arguments.insert(id.get_id(), ArgumentInstance::new(id.get_id(), argument));
        return Ok(());
    }

    pub (in crate) fn register_service_builder<S>(&mut self, service_id: ServiceId<S>,service_builder: ServiceBuilder) -> Result<(), Error> {
        if self.services.contains_key(service_id.get_id()) {
            return Err(Error::service_already_registered(service_id.get_id().to_string()));
        }
        let _ = self.services.insert(service_id.get_id(), service_builder);
        return Ok(());
    }

    pub fn build(self) -> DiContainer {
        return DiContainer::new(
            self.arguments,
            self.services,
        );
    }
}