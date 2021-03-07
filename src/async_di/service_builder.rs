use crate::service::service_instance::ServiceInstance;
use crate::service::service_name::ServiceName;
use crate::service::service::Service;
use crate::async_di::resolver::Resolver;
use std::any::Any;
use std::future::Future;
use futures::lock::Mutex;
use crate::error::Error;
use std::pin::Pin;
use std::clone::Clone;
use std::fmt;
mod service_builder_state;
use self::service_builder_state::ServiceBuilderState;

pub struct ServiceBuilder {
    service_name: ServiceName,
    factory: Box<dyn Fn(&Resolver) -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send>, Error>> + Send>> + Send>,
    configurators: Vec<Box<dyn Fn(&Resolver, &mut dyn Any) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> + Send>>,
    state: Mutex<ServiceBuilderState>,
}

impl ServiceBuilder {
    pub fn new(
        service_name: ServiceName,
        factory: Box<dyn Fn(&Resolver) -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send>, Error>> + Send>> + Send>,
    ) -> ServiceBuilder {
        return ServiceBuilder {
            service_name,
            factory,
            configurators: Vec::new(),
            state: Mutex::new(ServiceBuilderState::new()),
        }
    }

    pub fn register_configurator(&mut self, configurator: Box<dyn Fn(&Resolver, &mut dyn Any) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> + Send>) {
        self.configurators.push(configurator);
    }

    pub async fn resolve<T>(&self, resolver: &Resolver<'_>) -> Result<Service<T>, Error> 
    where
        T: Any + Send + 'static,
    {
        let mut state = self.state.lock().await;
        if let Some(service_instance) = state.get_service_instance() {
            return Ok(service_instance.as_service());
        }
        let service_factory = (self.factory)(resolver);
        let mut service = service_factory.await?;
        for configurator in self.configurators.iter() {
            let configurator_future = (configurator)(resolver, service.as_mut());
            configurator_future.await?;
        }
        let service_instance = ServiceInstance::new(self.service_name.clone(), service);
        state.store_service(service_instance.clone())?;
        return Ok(service_instance.as_service());
    }
}

impl fmt::Debug for ServiceBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.state.try_lock() {
            Some(state) => {
                format!("{:?}", state)
            },
            None => {
                "Locked".to_string()
            }
        };
        f.debug_struct("ServiceBuilder")
            .field("service_name", &self.service_name)
            .field("state", &state)
            .finish()
    }
}