// projects/libraries/common_time/src/timeout.rs
use crate::{Clock, TimeSpan};
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Instant;

/// A future that enforces a timeout on another future.
pub struct Timeout<F> {
    future: F,
    deadline: Instant,
    _pinned: PhantomPinned, // Ensure the struct is !Unpin
}

impl<F> Timeout<F> {
    pub fn new(future: F, clock: &impl Clock, timeout: TimeSpan) -> Self {
        let deadline = clock.now().into_std() + timeout.as_duration();
        Self {
            future,
            deadline,
            _pinned: PhantomPinned,
        }
    }
}

impl<F: Future> Future for Timeout<F> {
    type Output = Result<F::Output, &'static str>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = unsafe { self.get_unchecked_mut() }; // Safe because we don't move `future`
        if Instant::now() >= this.deadline {
            Poll::Ready(Err("Timeout exceeded"))
        } else {
            // Correctly pin the future before polling
            let future = unsafe { Pin::new_unchecked(&mut this.future) };
            future.poll(cx).map(Ok)
        }
    }
}

/// Wraps a future with a timeout.
pub fn with_timeout<F>(future: F, clock: &impl Clock, timeout: TimeSpan) -> Timeout<F> {
    Timeout::new(future, clock, timeout)
}
