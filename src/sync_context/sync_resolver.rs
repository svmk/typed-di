use crate::error::Error;
use crate::service::Service;
use crate::service_id::ServiceId;
use crate::argument_id::ArgumentId;
use crate::async_di_container::AsyncDiContainer;
use std::any::Any;
mod sync_resolve_future;
use self::sync_resolve_future::SyncResolveFuture;

#[derive(Debug, Clone)]
pub struct SyncResolver {
    container: AsyncDiContainer,
}

impl SyncResolver {
    pub fn new(
        container: AsyncDiContainer,
    ) -> SyncResolver {
        return SyncResolver {
            container,
        }
    }
    
    pub fn get_argument<T>(&self, id: ArgumentId<T>) -> Result<&T, Error> where T: Any, T: 'static {
        return self.container.get_argument(id);
    }

    pub fn get_service<T>(&self, id: ServiceId<T>) -> Result<Service<T>, Error>
        where T: Send + Any + 'static,
    {
        let future = self.container.clone().get_service(id);
        let (future, guard) = SyncResolveFuture::new(future);
        let _ = tokio::spawn(future);
        return guard.consume();
    }
}