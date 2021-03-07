use crate::error::Error;
use crate::service::service_access::ServiceAccess;
use crate::service::service_instance::ServiceInstance;
use std::fmt;
use std::sync::{Arc};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::clone::Clone;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Service<T>(Arc<ServiceInner<T>>);

enum ServiceReference<T> {
    TraitObject(T),
    Owned(PhantomData<T>),
}

impl <T>fmt::Debug for ServiceReference<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &ServiceReference::TraitObject(..) => {
                write!(f, "Trait object")
            },
            &ServiceReference::Owned(..) => {
                write!(f, "Owned")
            },
        }
    }
}

#[derive(Debug)]
struct ServiceInner<T> {
    instance: ServiceInstance,
    reference: ServiceReference<T>,
}

impl <T>Deref for Service<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match &self.0.reference {
            &ServiceReference::TraitObject(ref reference) => {
                return reference;
            },
            &ServiceReference::Owned(..) => {
                let reference = self.0.instance.as_ref_any();
                let reference: ::std::raw::TraitObject = unsafe {
                    ::std::mem::transmute(reference)
                };
                let reference = {
                    reference.data as *const T
                };
                let reference = unsafe {
                    match reference.as_ref() {
                        Some(reference) => reference,
                        None => {
                            panic!("Unable to deref service `{:?}`", self.0.instance.get_name());
                        },
                    }
                };
                return reference;
            }
        }
    }
}

/// TODO: Implement drop in separated thread.
unsafe impl <T>Sync for Service<T> {}
unsafe impl <T>Send for Service<T> {}

impl <T>Clone for Service<T> {
    fn clone(&self) -> Self {
        return Service(self.0.clone());
    }
}

impl <T>ServiceAccess<T> for Service<T> {
    fn from_instance(instance: ServiceInstance) -> Result<Service<T>, Error> where T: Any, T: 'static {
        if instance.get_type_id() == TypeId::of::<T>() {
            let service_inner = ServiceInner {
                instance,
                reference: ServiceReference::Owned(PhantomData),
            };
            let service = Arc::new(service_inner);
            let service = Service(service);
            return Ok(service);
        }
        return Err(Error::service_cast_error(instance.get_name().clone()))
    }

    fn from_trait_object(instance: ServiceInstance, trait_object: T) -> Service<T>
    {
        let service_ref = ServiceInner {
            instance,
            reference: ServiceReference::TraitObject(trait_object),
        };
        let service_ref = Service(Arc::new(service_ref));
        return service_ref;
    }
    fn get_instance(&self) -> ServiceInstance {
        return self.0.instance.clone();
    }

    fn as_ref(&self) -> &T {
        return self.deref();
    }
}