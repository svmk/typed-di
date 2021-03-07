use std::any::Any;

use crate::service::service::Service;
use crate::service::service_id::ServiceId;

pub trait ServiceConstructor<S> where S: Send + Sync + Any {
    fn new(service_id: ServiceId<S>, service: S) -> Service<S>;
}