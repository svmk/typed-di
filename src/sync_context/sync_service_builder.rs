use crate::service_instance::ServiceInstance;
use crate::sync_context::sync_resolver::SyncResolver;
use crate::error::{Error, BuildError};
use std::sync::{Arc, Mutex};
use std::clone::Clone;
use std::fmt;

#[derive(Clone, Debug)]
pub struct SyncServiceBuilder {
    service_id: &'static str,
    inner: Arc<Mutex<ServiceBuilderInner>>,
}

enum ServiceBuildingState {
    Builder(Box<dyn Fn(&SyncResolver) -> Result<ServiceInstance, Error> + Send>),
    Service(ServiceInstance),
}

impl fmt::Debug for ServiceBuildingState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ServiceBuildingState::Builder(..) => {
                write!(f, "Creating service")
            },
            &ServiceBuildingState::Service(..) => {
                write!(f, "Service is created")
            },
        }
    }
}

#[derive(Debug)]
struct ServiceBuilderInner {
    service: ServiceBuildingState,
}

impl SyncServiceBuilder {
    pub fn from_factory<S, F>(
        service_id: &'static str,
        factory: F,
    ) -> SyncServiceBuilder
        where
            S: Send + Sync + 'static,
            F: Fn(&SyncResolver) -> Result<S, BuildError> + Send + Sync + 'static,
    {
        let factory = move |context: &SyncResolver| {
            let service = (factory)(context).map_err(move |error| {
                return Error::service_build(service_id.to_string(), error);
            })?;
            let service = ServiceInstance::new(service_id, service);
            return Ok(service);
        };
        let factory = Box::new(factory);
        let inner = ServiceBuilderInner {
            service: ServiceBuildingState::Builder(factory),
        };
        let inner = Arc::new(Mutex::new(inner));
        return SyncServiceBuilder {
            service_id,
            inner,
        };
    }

    pub fn resolve(&self, context: &SyncResolver) -> Result<ServiceInstance, Error> {
        let mut inner = self
            .inner
            .lock()
            .expect("Unable to lock sync service builder");
        loop {
            match inner.service {
                ServiceBuildingState::Builder(ref builder) => {
                    let service = (builder)(context)?;
                    inner.service = ServiceBuildingState::Service(service);
                },
                ServiceBuildingState::Service(ref service) => {
                    return Ok(service.clone());
                },
            }
        }
    }
}