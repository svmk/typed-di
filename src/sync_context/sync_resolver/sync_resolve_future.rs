use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc;
use std::task::{Context, Poll};

pub struct SyncResolveGuard<T> {
    receiver: mpsc::Receiver<T>,
}

impl <T>SyncResolveGuard<T> {
    pub fn consume(self) -> T {
        return self
            .receiver
            .recv()
            .expect("Unable to consume future result");
    }
}

enum FutureState<T> {
    Pending {
        future: Pin<Box<dyn Future<Output=T> + Send>>,
        sender: mpsc::Sender<T>,
    },
    Complete,
}

pub struct SyncResolveFuture<T> {
    state: FutureState<T>,
}

impl <T>SyncResolveFuture<T>
    where T: Send + 'static,
{
    pub fn new(future: impl Future<Output=T> + Send + 'static) -> (SyncResolveFuture<T>, SyncResolveGuard<T>) {
        let future = Box::pin(future);
        let (sender, receiver) = mpsc::channel();
        let future = SyncResolveFuture {
            state: FutureState::Pending {
                future,
                sender,
            },
        };
        let guard = SyncResolveGuard {
            receiver,
        };
        return (future, guard);
    }
}

impl <T>Future for SyncResolveFuture<T> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        if let FutureState::Pending {ref mut future, ref sender} = self.state {
            let future = future.as_mut();
            let result = futures::ready!(future.poll(cx));
            if let Err(_) = sender.send(result) {
                panic!("Unable to send response to sync resolve guard");
            }
            self.state = FutureState::Complete;
        }
        return Poll::Ready(());
    }
}