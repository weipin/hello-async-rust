use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};
use std::thread::{self, Thread};

use crate::waker_fn;

/// Creates a Waker which, when waking, unparks a given thread.
fn waker_unparks(thread: Thread) -> Waker {
    waker_fn(move || thread.unpark())
}

/// Blocks the current thread on a future.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    let waker = waker_unparks(thread::current());
    let cx = &mut Context::from_waker(&waker);
    let mut future = pin!(future);
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}