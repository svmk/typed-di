use crate::service_instance::ServiceInstance;
use crate::sync_context::sync_resolver::SyncResolver;
use std::future::Future;
use futures::future::{FutureExt, TryFutureExt};
use crate::error::{Error, BuildError, JoinError};
use std::sync::{Arc, Mutex};
use std::pin::Pin;
use std::clone::Clone;
use std::fmt;
mod resolve_future;
pub use self::resolve_future::ResolveFuture;

#[derive(Clone, Debug)]
pub struct AsyncBlockingServiceBuilder {
    service_id: &'static str,
    inner: Arc<Mutex<ServiceBuilderInner>>,
}

enum ServiceBuildingState {
    FutureBuilder(Box<dyn Fn(&SyncResolver) -> Result<Pin<Box<dyn Future<Output=Result<ServiceInstance, Error>> + Send>>, Error> + Send>),
    ServiceFuturePending(Pin<Box<dyn Future<Output=Result<ServiceInstance, Error>> + Send>>),
    Service(ServiceInstance),
}

impl fmt::Debug for ServiceBuildingState {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ServiceBuildingState::FutureBuilder(..) => {
                write!(f, "Creating future")
            },
            &ServiceBuildingState::ServiceFuturePending(..) => {
                write!(f, "Pending future")
            },
            &ServiceBuildingState::Service(ref service) => {
                write!(f, "Service: {:?}", service)
            },
        }
    }
}

#[derive(Debug)]
struct ServiceBuilderInner {
    service: ServiceBuildingState,
}

impl AsyncBlockingServiceBuilder {
    pub fn from_async_factory<S, Fut>(service_id: &'static str, factory: impl Fn(&SyncResolver) -> Result<Fut, BuildError> + Send + Sync + 'static) -> AsyncBlockingServiceBuilder
        where
            S: Send + Sync + 'static,
            Fut: Future<Output=Result<S, BuildError>> + Send + 'static,
    {
        let factory = Arc::new(factory);
        let factory = move |context: &SyncResolver| {
            let context = context.clone();
            let thread_factory = factory.clone(); 
            let future = tokio::task::spawn_blocking(move || -> Result<Fut, BuildError> {
                return (thread_factory)(&context);
            });
            let future = future
                .map_err(move |error| {
                    return Error::join_error(JoinError::new(error));
                })
                .and_then(move |service_result| {
                    let service_result = service_result.map_err(|error| {
                        return Error::service_build(service_id.to_string(), error);
                    });
                    match service_result {
                        Ok(service_result_future) => {
                            return service_result_future.map(move |service| -> Result<ServiceInstance, Error> {
                                let service = service.map_err(|error|{
                                    return Error::service_build(service_id.to_string(), error);
                                })?;
                                let service = ServiceInstance::new(service_id, service);
                                return Ok(service);
                            }).boxed();
                        },
                        Err(error) => {
                            return futures::future::err(error).boxed();
                        },
                    }
                });
            // let future = future.map_err(move |error| {
            //     return Error::service_build(service_id.to_string(), error);
            // });
            let future: Pin<Box<dyn Future<Output=Result<ServiceInstance, Error>> + Send>> = Box::pin(future);
            return Ok(future);
        };
        let factory = Box::new(factory);
        let inner = ServiceBuilderInner {
            service: ServiceBuildingState::FutureBuilder(factory),
        };
        let inner = Arc::new(Mutex::new(inner));
        return AsyncBlockingServiceBuilder {
            service_id,
            inner,
        };
    }

    pub fn from_future<S>(service_id: &'static str, factory: impl Future<Output=Result<S, BuildError>> + Send + 'static) -> AsyncBlockingServiceBuilder
        where
            S: Send + Sync + 'static,
    {
        let factory = factory.map(move |service_result| {
            return service_result.map(move |service| {
                return ServiceInstance::new(service_id, service);
            }).map_err(|error| {
                return Error::service_build(service_id.to_string(), error);
            });
        });
        let inner = ServiceBuilderInner {
            service: ServiceBuildingState::ServiceFuturePending(Box::pin(factory)),
        };
        let inner = Arc::new(Mutex::new(inner));
        return AsyncBlockingServiceBuilder {
            service_id,
            inner,
        };
    }

    pub fn resolve(self, context: SyncResolver) -> ResolveFuture {
        return ResolveFuture::new(self, context);
    }
}