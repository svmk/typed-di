use crate::error::Error;
use crate::service::service_instance::ServiceInstance;

#[derive(Debug)]
pub struct ServiceBuilderState {
    service_instance: Option<ServiceInstance>,
}

impl ServiceBuilderState {
    pub fn new() -> ServiceBuilderState {
        return ServiceBuilderState {
            service_instance: None,
        }
    }

    pub fn get_service_instance(&self) -> Option<ServiceInstance> {
        if let Some(ref service_instance) = self.service_instance {
            return Some(service_instance.clone());
        }
        return None;
    }

    pub fn store_service(&mut self, service_instance: ServiceInstance) -> Result<(), Error> {
        if self.service_instance.is_some() {
            return Err(Error::service_already_registered(service_instance.get_name().clone()));
        }
        self.service_instance = Some(service_instance);
        return Ok(());
    }
}