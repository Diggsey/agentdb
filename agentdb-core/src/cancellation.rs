use std::{future::Future, task::Poll};

use futures::{future::FusedFuture, pin_mut, ready, FutureExt};
use tokio::{
    sync::watch,
    task::{JoinError, JoinHandle},
};

#[derive(Debug, Clone)]
pub struct Cancellation {
    rx: watch::Receiver<bool>,
    cancelled: bool,
}

impl Cancellation {
    pub fn is_cancelled(&self) -> bool {
        *self.rx.borrow()
    }
}

impl Future for Cancellation {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if !*self.rx.borrow_and_update() {
            let changed = self.rx.changed();
            pin_mut!(changed);
            ready!(changed.poll(cx)).ok();
        }
        self.cancelled = true;
        Poll::Ready(())
    }
}

impl FusedFuture for Cancellation {
    fn is_terminated(&self) -> bool {
        self.cancelled
    }
}

pub struct CancellableHandle<T> {
    tx: watch::Sender<bool>,
    inner: JoinHandle<T>,
}

impl<T> CancellableHandle<T> {
    pub fn cancel(&self) {
        let _ = self.tx.send(true);
    }
    pub fn forget(self) -> JoinHandle<T> {
        let Self { tx, inner } = self;
        tokio::spawn(async move {
            tx.closed().await;
        });
        inner
    }
}

impl<T> Future for CancellableHandle<T> {
    type Output = Result<T, JoinError>;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        {
            // Wait for all cancellations to be dropped
            let closed = self.tx.closed();
            pin_mut!(closed);
            ready!(closed.poll(cx));
        }
        self.inner.poll_unpin(cx)
    }
}

pub fn spawn_cancellable<T: Send + 'static, F: Future<Output = T> + Send + 'static>(
    f: impl FnOnce(Cancellation) -> F,
) -> CancellableHandle<T> {
    let (tx, rx) = watch::channel(false);
    let inner = tokio::spawn(f(Cancellation {
        rx,
        cancelled: false,
    }));
    CancellableHandle { tx, inner }
}
