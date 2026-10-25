use serde_json::Value;

/// A call that failed, as the plugin decides. Only the message is required; the code, the retry
/// hint and the details are sent to the host when there are any.
#[derive(Debug, Clone, PartialEq)]
pub struct MethodFailure {
    pub message: String,
    /// What kind of failure: a number with the meaning of the HTTP status code of that number
    /// (400, 404, 429, 503), or a name of the plugin's own (`MODEL_MISSING`).
    pub code: Option<Value>,
    /// Whether trying again could work.
    pub retryable: Option<bool>,
    /// Anything else worth keeping, such as what the wrapped service answered.
    pub details: Option<Value>,
}

impl MethodFailure {
    /// A failure with the meaning of an HTTP status code. The host knows what the standard codes
    /// mean (whether a retry could work, for one), so nothing more needs saying.
    pub fn status(code: u16, message: impl Into<String>) -> Self {
        MethodFailure {
            message: message.into(),
            code: Some(Value::from(code)),
            retryable: None,
            details: None,
        }
    }
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
