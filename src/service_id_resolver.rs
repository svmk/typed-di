use crate::service_id::ServiceId;
use std::any::{type_name, Any};

pub trait ServiceIdResolver: Sized {
    const SERVICE_ID: ServiceId<Self> = ServiceId::new(type_name::<Self>());
}

impl <T>ServiceIdResolver for T where T: Any {

}