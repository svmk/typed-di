use crate::error::BuildError;
use crate::sync_context::sync_resolver::SyncResolver;
use crate::sync_context::sync_service_builder::SyncServiceBuilder;
use crate::container_declaration::ContainerDeclaration;
use crate::service_id::ServiceId;

pub trait SyncContainerDeclaration {
    fn register<S, F>(&mut self, service_id: ServiceId<S>, factory: F) -> Result<(), BuildError>
        where
            S: Send + Sync + 'static,
            F: Fn(&SyncResolver) -> Result<S, BuildError> + Send + Sync + 'static;
}

impl SyncContainerDeclaration for ContainerDeclaration {
    fn register<S, F>(&mut self, service_id: ServiceId<S>, factory: F) -> Result<(), BuildError> where
        S: Send + Sync + 'static,
        F: Fn(&SyncResolver) -> Result<S, BuildError> + Send + Sync + 'static,
    {
        let service_builder = SyncServiceBuilder::from_factory(
            service_id.get_id(),
            factory,
        );
        self.register_service_builder(service_id, service_builder.into())?;
        return Ok(());
    }
}