use crate::service::service_id::ServiceId;
use crate::service::service_instance::ServiceInstance;
use crate::service::service::Service;
use crate::service::service_access::ServiceAccess;
use std::any::Any;
use std::ops::Deref;

const MD5_HAHSER_SERVICE: ServiceId<Md5Hasher> = ServiceId::typename();
struct Md5Hasher {
    _count: usize,
}

impl Md5Hasher {
    pub fn new(count: usize) -> Md5Hasher {
        return Md5Hasher {
            _count: count,
        }
    }
}


#[test]
fn test_owned_service_ptr() {
    let hasher = Md5Hasher::new(100_000);
    let hasher: Box<dyn Any> = Box::new(hasher);
    let hasher_ptr = format!("{:p}", hasher);
    let service_instance = ServiceInstance::new(MD5_HAHSER_SERVICE.get_name().clone(), hasher);
    let service: Service<Md5Hasher> = Service::from_instance(service_instance);
    let service_ref = service.deref();
    let service_ptr = format!("{:p}", service_ref);
    assert_eq!(hasher_ptr, service_ptr);
}