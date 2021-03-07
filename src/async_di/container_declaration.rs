use crate::error::{BuildErrorCast};
use crate::service::service_id::ServiceId;
use crate::service::service_name::ServiceName;
use crate::async_di::service_builder::ServiceBuilder;
use crate::async_di::resolver::Resolver;
use crate::error::Error;
use crate::argument::argument_instance::ArgumentInstance;
use crate::argument::argument_id::ArgumentId;
use crate::argument::argument_name::ArgumentName;
use crate::async_di::container::Container;
use std::collections::HashMap;
use std::future::{Future, ready as ready_future};
use futures::FutureExt;
use std::any::Any;
use std::pin::Pin;

#[derive(Debug)]
pub struct ContainerDeclaration {
    arguments: HashMap<ArgumentName, ArgumentInstance>,
    services: HashMap<ServiceName, ServiceBuilder>,
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
        if self.arguments.contains_key(id.get_name()) {
            return Err(Error::argument_already_registered(id.get_name().clone()));
        }
        let _ = self.arguments.insert(id.get_name().clone(), ArgumentInstance::new(id.get_name().clone(), argument));
        return Ok(());
    }

    pub fn register<S, Fut, F, E>(&mut self, service_id: ServiceId<S>, factory: F) -> Result<(), Error>
        where
            F: Fn(&Resolver) -> Fut + Send + Sync + 'static,
            S: Send + Sync + 'static,
            E: BuildErrorCast + 'static,
            Fut: Future<Output=Result<S, E>> + Send + 'static {
                let service_name = service_id.get_name().clone();
                let factory = move |resolver: &Resolver| -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send>, Error>> + Send>> {
                    let future = (factory)(resolver);
                    let service_name = service_name.clone();
                    let future = future.map(move |service| {
                        match service {
                            Ok(service) => {
                                let service: Box<dyn Any + Send> = Box::new(service);
                                return Ok(service)
                            },
                            Err(error) => {
                                let error = error.into_build_error();
                                let error = Error::service_build(service_name.clone(), error);
                                return Err(error);
                            },
                        }
                    });
                    return Box::pin(future);
                };
                let factory: Box<dyn Fn(&Resolver) -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send>, Error>> + Send>> + Send> = Box::new(factory);
                let service_builder = ServiceBuilder::new(
                    service_id.get_name().clone(),
                    factory,
                );
                self.register_service_builder(service_id, service_builder)?;
                return Ok(());
            }

    pub fn register_ready<S, Fut, F>(&mut self, service_id: ServiceId<S>, factory: F) -> Result<(), Error>
        where
            F: Fn(&Resolver) -> Fut + Send + Sync + 'static,
            S: Send + Sync + 'static,
            Fut: Future<Output=S> + Send + 'static {
                let factory = move |resolver: &Resolver| -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send>, Error>> + Send>> {
                    let future = (factory)(resolver);
                    let future = future.map(move |service| -> Result<Box<dyn Any + Send>, Error> {
                        let service: Box<dyn Any + Send> = Box::new(service);
                        return Ok(service)
                    });
                    return Box::pin(future);
                };
                let factory: Box<dyn Fn(&Resolver) -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send>, Error>> + Send>> + Send> = Box::new(factory);
                let service_builder = ServiceBuilder::new(
                    service_id.get_name().clone(),
                    factory,
                );
                self.register_service_builder(service_id, service_builder)?;
                return Ok(());
            }

    pub fn configure<S, Fut, F, E>(&mut self, service_id: ServiceId<S>, configurator: F) -> Result<(), Error>
        where
            F: Fn(&Resolver, &mut S) -> Fut + Send + Sync + 'static,
            S: Send + Sync + Any + 'static,
            E: BuildErrorCast + 'static,
            Fut: Future<Output=Result<(), E>> + Send + 'static {
                let service_name = service_id.get_name().clone();
                let configurator = move |resolver: &Resolver, service: &mut dyn Any| -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> {
                    let service = match service.downcast_mut::<S>() {
                        Some(service) => service,
                        None => {
                            let future = ready_future(Err(Error::service_cast_error(service_name.clone())));
                            return Box::pin(future);                            
                        },
                    };
                    let future = (configurator)(resolver, service);
                    let service_name = service_name.clone();
                    let future = future.map(move |result: Result<(), E>| {
                        match result {
                            Ok(()) => {
                                return Ok(());
                            },
                            Err(error) => {
                                let error = error.into_build_error();
                                let error = Error::service_build(service_name.clone(), error);
                                return Err(error);
                            },
                        }
                    });
                    return Box::pin(future);
                };
                let configurator: Box<dyn Fn(&Resolver, &mut dyn Any) -> Pin<Box<dyn Future<Output=Result<(), Error>> + Send>> + Send> = Box::new(configurator);
                let service_builder = self.get_mut_service_builder(service_id.get_name())?;
                service_builder.register_configurator(configurator);
                return Ok(());
            }
            
    fn get_mut_service_builder(&mut self, service_name: &ServiceName) -> Result<&mut ServiceBuilder, Error> {
        if let Some(service) = self.services.get_mut(service_name) {
            return Ok(service);
        }
        return Err(Error::service_builder_not_found(service_name.clone()));
    }

    fn register_service_builder<S>(&mut self, service_id: ServiceId<S>, service_builder: ServiceBuilder) -> Result<(), Error> {
        if self.services.contains_key(service_id.get_name()) {
            return Err(Error::service_already_registered(service_id.get_name().clone()));
        }
        let _ = self.services.insert(service_id.get_name().clone(), service_builder);
        return Ok(());
    }

    pub fn build(self) -> Container {
        return Container::new(
            self.arguments,
            self.services,
        );
    }
}