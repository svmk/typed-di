use crate::async_blocking_context::async_blocking_service_builder::AsyncBlockingServiceBuilder;
use crate::sync_context::sync_service_builder::SyncServiceBuilder;

#[derive(Clone, Debug)]
pub enum ServiceBuilder {
    AsyncBlocking(AsyncBlockingServiceBuilder),
    Sync(SyncServiceBuilder),
}

impl From<AsyncBlockingServiceBuilder> for ServiceBuilder {
    fn from(builder: AsyncBlockingServiceBuilder) -> Self {
        return ServiceBuilder::AsyncBlocking(builder);
    }
}

impl From<SyncServiceBuilder> for ServiceBuilder {
    fn from(builder: SyncServiceBuilder) -> Self {
        return ServiceBuilder::Sync(builder);
    }
}