use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// What an action holds of its call besides the params. It tells the action that the host has
/// cancelled the call (a long action looks at it now and then and stops early; answering a
/// cancelled call is optional, and the host ignores the answer), and it lets the action say it is
/// alive with [`Cancel::progress`].
#[derive(Clone, Default)]
pub struct Cancel {
    flag: Arc<AtomicBool>,
    progress: Option<Arc<dyn Fn(&str) + Send + Sync>>,
}

impl std::fmt::Debug for Cancel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cancel")
            .field("cancelled", &self.is_cancelled())
            .finish()
    }
}

impl Cancel {
    pub fn is_cancelled(&self) -> bool {
        self.flag.load(Ordering::SeqCst)
    }

    /// Tells the host the call is alive and, in a few words, what it is doing. A plugin whose
    /// manifest sets `stallSeconds` must do this more often than that, or the host stops it. Does
    /// nothing outside a served call.
    pub fn progress(&self, message: &str) {
        if let Some(send) = &self.progress {
            send(message);
        }
    }

    /// A token whose `progress` calls `send`.
    pub(crate) fn with_progress(send: impl Fn(&str) + Send + Sync + 'static) -> Cancel {
        Cancel {
            flag: Arc::default(),
            progress: Some(Arc::new(send)),
        }
    }

    pub(crate) fn cancel(&self) {
        self.flag.store(true, Ordering::SeqCst);
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

    #[test]
    fn progress_does_nothing_outside_a_served_call_and_calls_the_sender_inside_one() {
        Cancel::default().progress("nobody listens");
        let heard = Arc::new(std::sync::Mutex::new(Vec::new()));
        let sink = Arc::clone(&heard);
        let cancel =
            Cancel::with_progress(move |message| sink.lock().unwrap().push(message.to_string()));
        cancel.clone().progress("half way");
        assert_eq!(*heard.lock().unwrap(), ["half way"]);
    }
}
