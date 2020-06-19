use crate::error::Error;
use crate::service::Service;
use crate::service_access::ServiceAccess;
use std::sync::{Arc};
use std::any::{Any, TypeId, type_name};
use std::clone::Clone;

#[derive(Debug)]
pub struct ServiceInstance {
    type_id: TypeId,
    service_type: &'static str,
    service_id: &'static str,
    service: Arc<Box<dyn Any>>,
}

impl ServiceInstance {
    pub fn new<T>(service_id: &'static str, service: T) -> ServiceInstance
        where
            T: 'static,
            T: Any,
            T: Send,
            T: Sync,
    {
        return ServiceInstance {
            type_id: TypeId::of::<T>(),
            service_type: type_name::<T>(),
            service_id,
            service: Arc::new(Box::new(service)),
        }
    }

    pub fn get_id(&self) -> &'static str {
        return self.service_id;
    }

    pub fn as_service<S>(self) -> Result<Service<S>, Error> where S: Any, S: 'static {
        return Service::from_instance(self);
    }

    pub fn get_service_type(&self) -> &'static str {
        return self.service_type;
    }

    pub fn get_type_id(&self) -> TypeId {
        return self.type_id;
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
            type_id: self.type_id,
            service_id: self.service_id,
            service_type: self.service_type,
            service: self.service.clone(),
        }
    }
}