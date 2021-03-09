use crate::async_di::service_builder::ServiceBuilder;
use crate::service::service_name::ServiceName;
use crate::async_di::resolver::Resolver;
use crate::async_di::container::Container;
use crate::error::Error;
use std::collections::HashMap;
use std::pin::Pin;
use std::future::Future;
use std::any::Any;

const SERVICE_NAME: ServiceName = ServiceName::StaticString("test_service");

fn ensure_send_sync<T>(_value: T) where T: Send, T: Sync, {}

#[test]
fn test_service_builder_send_sync() {
    let factory = |_resolver: Resolver| -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send + Sync>, Error>> + Send + Sync>> {
        panic!("not required");
    };
    let factory = Box::new(factory);
    let service_builder = ServiceBuilder::new(SERVICE_NAME.clone(), factory);
    ensure_send_sync(service_builder);
}

#[test]
fn test_service_builder_resolver_send_sync() {
    let factory = |_resolver: Resolver| -> Pin<Box<dyn Future<Output=Result<Box<dyn Any + Send + Sync>, Error>> + Send + Sync>> {
        panic!("not required");
    };
    let factory = Box::new(factory);
    let service_builder = ServiceBuilder::new(SERVICE_NAME.clone(), factory);
    let container = Container::new(HashMap::new(), HashMap::new());
    let resolver = Resolver::new(container);
    let future = service_builder.resolve::<()>(resolver);
    ensure_send_sync(future);
}