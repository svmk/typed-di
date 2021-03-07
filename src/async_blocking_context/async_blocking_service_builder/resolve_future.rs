use crate::error::Error;
use crate::async_blocking_context::async_blocking_service_builder::{AsyncBlockingServiceBuilder,ServiceBuildingState};
use crate::service_instance::ServiceInstance;
use crate::sync_context::sync_resolver::SyncResolver;
use futures::ready;
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

#[derive(Debug)]
pub struct ResolveFuture {
    builder: AsyncBlockingServiceBuilder,
    context: SyncResolver,
}

impl ResolveFuture {
    pub fn new(
        builder: AsyncBlockingServiceBuilder,
        context: SyncResolver,
    ) -> ResolveFuture {
        return ResolveFuture {
            builder,
            context,
        }
    }
}

impl Future for ResolveFuture {
    type Output = Result<ServiceInstance, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        let mut inner = self
            .builder
            .inner
            .lock()
            .expect("Unable to lock async blocking service builder");
        loop {
            match inner.service {
                ServiceBuildingState::FutureBuilder(ref builder) => {
                    let future = match builder.create_future(&self.context) {
                        Ok(future) => {future},
                        Err(error) => {
                            return Poll::Ready(Err(error));
                        },
                    };
                    inner.service = ServiceBuildingState::ServiceFuturePending(future);
                },
                ServiceBuildingState::ServiceFuturePending(ref mut future) => {
                    let future = future.as_mut();
                    let service = ready!(future.poll(cx));
                    let service = match service {
                        Ok(service) => service,
                        Err(error) => {
                            return Poll::Ready(Err(error));
                        },
                    };
                    inner.service = ServiceBuildingState::Service(service);
                },
                ServiceBuildingState::Service(ref service) => {
                    return Poll::Ready(Ok(service.clone()));
                },
            }
        }
    }
}