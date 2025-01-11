use std::sync::Arc;
use std::task::{Wake, Waker};

/// Converts a closure into a Waker.
///
/// Copied from: https://github.com/smol-rs/waker-fn/blob/master/src/lib.rs
pub fn waker_fn<F: Fn() + Send + Sync + 'static>(f: F) -> Waker {
    Waker::from(Arc::new(Helper(f)))
}

struct Helper<F>(F);

impl<F: Fn() + Send + Sync + 'static> Wake for Helper<F> {
    fn wake(self: Arc<Self>) {
        (self.0)();
    }

    fn wake_by_ref(self: &Arc<Self>) {
        (self.0)();
    }
}
