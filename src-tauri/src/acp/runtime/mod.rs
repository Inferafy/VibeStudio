use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Default)]
pub struct AcpRuntime {
    active_connection_count: AtomicUsize,
}

impl AcpRuntime {
    #[must_use]
    pub fn active_connection_count(&self) -> usize {
        self.active_connection_count.load(Ordering::Acquire)
    }
}
