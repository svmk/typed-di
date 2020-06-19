use crate::error::{Error, JoinError};
use crate::service_builder::ServiceBuilder;
use crate::service_instance::ServiceInstance;
use crate::service::Service;
use crate::service_id::ServiceId;
use crate::argument_id::ArgumentId;
use crate::sync_context::sync_resolver::SyncResolver;
use crate::di_container::DiContainer;
use futures::future::FutureExt;
use std::sync::Arc;
use std::any::Any;

#[derive(Debug, Clone)]
pub struct AsyncDiContainer {
    container: Arc<DiContainer>,
}

impl AsyncDiContainer {
    pub fn new(container: DiContainer) -> AsyncDiContainer {
        return AsyncDiContainer {
            container: Arc::new(container),
        }
    }

    pub fn get_argument<T>(&self, id: ArgumentId<T>) -> Result<&T, Error> where T: Any, T: 'static {
        return self.container.get_argument(id);
    }

    pub async fn get_service<T>(self, id: ServiceId<T>) -> Result<Service<T>, Error>
        where
            T: Any + Send + 'static,
    {
        let id = id.get_id();
        let service_instance = self.get_service_instance(id).await?;
        return service_instance.as_service();
    }

    async fn get_service_instance(self, id: &str) -> Result<ServiceInstance, Error> {
        let service = self.container.get_service_builder(id)?;
        let context = SyncResolver::new(self.clone());
        let service = service.clone();
        let resolve_future = tokio::task::spawn_blocking(move || {
            match service {
                ServiceBuilder::AsyncBlocking(service) => {
                    let service_instance_future = service.resolve(context).boxed();
                    return service_instance_future;
                },
                ServiceBuilder::Sync(service) => {
                    let service_instance_result = service.resolve(&context);
                    let service_instance_future = futures::future::ready(service_instance_result).boxed();
                    return service_instance_future;
                },
            }
        });
        let resolve_future = resolve_future.await;
        let resolve_future = resolve_future.map_err(|error| {
            return Error::join_error(JoinError::new(error));
        })?;
        let service_instance = resolve_future.await?;
        return Ok(service_instance);
    }
}