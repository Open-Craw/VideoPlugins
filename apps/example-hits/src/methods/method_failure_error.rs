use serde_json::Value;

/// A call that failed, as the plugin decides. Only the message is required; the code, the retry
/// hint and the details are sent to the host when there are any.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodFailure {
    pub message: String,
    /// A short code a flow can test, such as `RATE_LIMITED`.
    pub code: Option<String>,
    /// Whether trying again could work.
    pub retryable: Option<bool>,
    /// Anything else worth keeping, such as what the wrapped service answered.
    pub details: Option<Value>,
}

impl From<String> for MethodFailure {
    fn from(message: String) -> Self {
        MethodFailure {
            message,
            code: None,
            retryable: None,
            details: None,
        }
    }
}

impl From<&str> for MethodFailure {
    fn from(message: &str) -> Self {
        message.to_string().into()
    }
}
