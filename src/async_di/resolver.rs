use crate::error::Error;
use crate::service::service::Service;
use crate::service::service_id::ServiceId;
use crate::argument::argument_id::ArgumentId;
use crate::async_di::container::Container;
use std::any::Any;

#[derive(Debug)]
pub struct Resolver<'a> {
    container: &'a Container,
}

impl Resolver<'_> {
    pub fn new(
        container: &'_ Container,
    ) -> Resolver {
        return Resolver {
            container,
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