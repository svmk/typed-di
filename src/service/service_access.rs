use crate::error::Error;
use crate::service::service_instance::ServiceInstance;
use crate::service::service::Service;
use std::any::Any;

pub trait ServiceAccess<T> {
    fn from_instance(instance: ServiceInstance) -> Result<Service<T>, Error> where T: Any, T: 'static;
    fn from_trait_object(instance: ServiceInstance, trait_object: T) -> Service<T>;
    fn get_instance(&self) -> ServiceInstance;
    fn as_ref(&self) -> &T;
}