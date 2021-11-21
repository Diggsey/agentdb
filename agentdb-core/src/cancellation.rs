use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use futures::{future::FusedFuture, FutureExt};
use tokio::{
    sync::{OwnedRwLockReadGuard, OwnedRwLockWriteGuard, RwLock},
    task::{JoinError, JoinHandle},
};

pub struct Cancellation {
    inner: Arc<RwLock<bool>>,
    fut: Pin<Box<dyn Future<Output = OwnedRwLockReadGuard<bool>> + Send + Sync>>,
}

impl Cancellation {
    pub fn is_cancelled(&self) -> bool {
        matches!(self.inner.try_read(), Ok(x) if *x)
    }
}

impl Clone for Cancellation {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            fut: Box::pin(self.inner.clone().read_owned()),
        }
    }
}

impl Future for Cancellation {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.fut.poll_unpin(cx) {
            Poll::Ready(x) if *x => Poll::Ready(()),
            _ => Poll::Pending,
        }
    }
}

impl FusedFuture for Cancellation {
    fn is_terminated(&self) -> bool {
        false
    }
}

pub struct CancellableHandle<T> {
    guard: Option<OwnedRwLockWriteGuard<bool>>,
    inner: JoinHandle<T>,
}

impl<T> CancellableHandle<T> {
    pub fn cancel(&mut self) {
        self.guard.take();
    }
    pub fn forget(self) -> JoinHandle<T> {
        let Self { guard, inner } = self;
        if let Some(mut guard) = guard {
            // Don't trigger cancellation when we release the write lock
            *guard = false;
        }
        inner
    }
}

impl<T> Future for CancellableHandle<T> {
    type Output = Result<T, JoinError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.inner.poll_unpin(cx)
    }
}

pub fn spawn_cancellable<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
    f: impl FnOnce(Cancellation) -> F,
) -> CancellableHandle<T> {
    let rwlock = Arc::new(RwLock::new(true));
    let write_guard = rwlock
        .clone()
        .try_write_owned()
        .expect("RwLock can be immediately locked");
    let read_guard = Box::pin(rwlock.clone().read_owned());
    let inner = tokio::spawn(f(Cancellation {
        inner: rwlock,
        fut: read_guard,
    }));
    CancellableHandle {
        guard: Some(write_guard),
        inner,
    }
}
