use crate::service::service_id::ServiceId;
use std::any::{Any};

pub trait ServiceIdResolver: Sized + 'static {
    const SERVICE_ID: ServiceId<Self> = ServiceId::typename();
}

impl <T>ServiceIdResolver for T where T: Any {

}