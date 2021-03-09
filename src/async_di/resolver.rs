use crate::error::Error;
use crate::service::service::Service;
use crate::service::service_id::ServiceId;
use crate::argument::argument_id::ArgumentId;
use crate::async_di::container::Container;
use std::any::Any;

#[cfg(test)]
mod resolver_test;

#[derive(Debug)]
pub struct Resolver {
    container: Container,
}

impl Resolver {
    pub fn new(
        container: Container,
    ) -> Resolver {
        return Resolver {
            container,
        }
    }

    pub (crate) fn resolver_clone(&self) -> Resolver {
        return Resolver {
            container: self.container.container_clone(),
        }
    }
    
    pub fn get_argument<T>(&self, id: ArgumentId<T>) -> Result<&T, Error> where T: Any, T: 'static {
        return self.container.get_argument(id);
    }

    pub async fn get_service<T>(&self, id: ServiceId<T>) -> Result<Service<T>, Error>
        where T: Send + Any + 'static,
    {
        let future = self.container.get_service(id);
        return future.await;
    }
}