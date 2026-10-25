use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Tells a call that the host has cancelled it. A long action looks at it now and then and stops
/// early; answering a cancelled call is optional, and the host ignores the answer.
#[derive(Debug, Clone, Default)]
pub struct Cancel(Arc<AtomicBool>);

impl Cancel {
    pub fn is_cancelled(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }

    pub(crate) fn cancel(&self) {
        self.0.store(true, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_clone_sees_the_cancel() {
        let cancel = Cancel::default();
        let seen_by_the_call = cancel.clone();
        assert!(!seen_by_the_call.is_cancelled());
        cancel.cancel();
        assert!(seen_by_the_call.is_cancelled());
    }
}
