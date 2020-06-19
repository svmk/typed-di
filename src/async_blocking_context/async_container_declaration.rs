use crate::error::BuildError;
use crate::service_id::ServiceId;
use crate::async_blocking_context::async_blocking_service_builder::AsyncBlockingServiceBuilder;
use crate::sync_context::sync_resolver::SyncResolver;
use crate::container_declaration::ContainerDeclaration;
use std::future::Future;
pub trait AsyncContainerDeclaration {
    fn register_async_blocking<S, Fut, F>(&mut self, service_id: ServiceId<S>, factory: F) -> Result<(), BuildError>
        where
            F: Fn(&SyncResolver) -> Result<Fut, BuildError> + Send + Sync + 'static,
            S: Send + Sync + 'static,
            Fut: Future<Output=Result<S, BuildError>> + Send + 'static;

    fn register_future_blocking<S, F>(&mut self, service_id: ServiceId<S>, future: F) -> Result<(), BuildError>
        where
            F: Future<Output=Result<S, BuildError>> + Send + 'static,
            S: Send + Sync + 'static;
}

impl AsyncContainerDeclaration for ContainerDeclaration {
    fn register_async_blocking<S, Fut, F>(&mut self, service_id: ServiceId<S>, factory: F) -> Result<(), BuildError>
        where
            F: Fn(&SyncResolver) -> Result<Fut, BuildError> + Send + Sync + 'static,
            S: Send + Sync + 'static,
            Fut: Future<Output=Result<S, BuildError>> + Send + 'static,
    {
        let service_builder = AsyncBlockingServiceBuilder::from_async_factory(service_id.get_id(), factory);
        self.register_service_builder(service_id, service_builder.into())?;
        return Ok(());
    }

    fn register_future_blocking<S, F>(&mut self, service_id: ServiceId<S>, future: F) -> Result<(), BuildError>
        where
            F: Future<Output=Result<S, BuildError>> + Send + 'static,
            S: Send + Sync + 'static,
    {

        let service_builder = AsyncBlockingServiceBuilder::from_future(service_id.get_id(), future);
        self.register_service_builder(service_id, service_builder.into())?;
        return Ok(());
    }
}