use crate::error::Error;
use crate::service::service::Service;
use crate::service::service_access::ServiceAccess;
use crate::service::service_name::ServiceName;
use std::sync::{Arc};
use std::any::{Any, TypeId};
use std::clone::Clone;
#[cfg(test)]
mod service_instance_test;

#[derive(Debug)]
pub struct ServiceInstance {
    service_name: ServiceName,
    service: Arc<Box<dyn Any>>,
}

impl ServiceInstance {
    pub fn new(service_name: ServiceName, service: Box<dyn Any>) -> ServiceInstance
        where
    {
        return ServiceInstance {
            service_name,
            service: Arc::new(service),
        }
    }

    pub fn get_name(&self) -> &ServiceName {
        return &self.service_name;
    }

    pub fn as_service<S>(self) -> Service<S> where S: Any, S: 'static {
        return Service::from_instance(self);
    }

    pub fn as_ref_any(&self) -> &dyn Any {
        let service = self.service.as_ref() as &Box<dyn Any>;
        return service.as_ref();
    }
}

unsafe impl Sync for ServiceInstance {}
unsafe impl Send for ServiceInstance {}

impl Clone for ServiceInstance {
    fn clone(&self) -> Self {
        return ServiceInstance {
            service_name: self.service_name.clone(),
            service: self.service.clone(),
        }
    }
}
