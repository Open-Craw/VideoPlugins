use serde_json::Value;

use super::method_failure_error::MethodFailure;

/// Decides what a wrapped service's answer means. This is the plugin's job: the host never reads a
/// result to find out whether it failed.
///
/// - An answer with `"error": true` is a failure even when its `status` is 200, as some services
///   do. The failure keeps the service's `code` and `retryable` when it gave them, and the whole
///   answer as its details.
/// - A `status` of 429 or 503 is a failure that could work again; any other status of 400 or more
///   is one that could not.
/// - `null` is a failure too: a service that answers nothing has not done its job.
/// - Anything else is a success, and the answer is the result.
pub fn interpret_response(response: &Value) -> Result<Value, MethodFailure> {
    if response.is_null() {
        return Err(MethodFailure {
            message: "the service answered nothing".to_string(),
            code: Some("EMPTY_RESPONSE".to_string()),
            retryable: Some(true),
            details: None,
        });
    }
    let status = response["status"].as_u64().unwrap_or(200);
    let flagged = response["error"].as_bool().unwrap_or(false);
    if flagged || status >= 400 {
        let busy = matches!(status, 429 | 503);
        return Err(MethodFailure {
            message: response["message"]
                .as_str()
                .map_or_else(|| format!("the service answered {status}"), str::to_string),
            code: Some(
                response["code"]
                    .as_str()
                    .unwrap_or(if flagged {
                        "SERVICE_ERROR"
                    } else {
                        "HTTP_ERROR"
                    })
                    .to_string(),
            ),
            retryable: Some(response["retryable"].as_bool().unwrap_or(busy)),
            details: Some(response.clone()),
        });
    }
    Ok(response.clone())
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn a_200_with_error_true_is_a_failure_that_keeps_what_the_service_said() {
        let response = json!({ "status": 200, "error": true, "message": "quota", "code": "QUOTA" });
        let failure = interpret_response(&response).unwrap_err();
        assert_eq!(failure.message, "quota");
        assert_eq!(failure.code.as_deref(), Some("QUOTA"));
        assert_eq!(failure.retryable, Some(false));
        assert_eq!(failure.details, Some(response));
    }

    #[test]
    fn busy_statuses_can_be_retried_and_other_error_statuses_cannot() {
        let busy = interpret_response(&json!({ "status": 429 })).unwrap_err();
        assert_eq!(busy.retryable, Some(true));
        assert_eq!(busy.code.as_deref(), Some("HTTP_ERROR"));
        let bad = interpret_response(&json!({ "status": 400 })).unwrap_err();
        assert_eq!(bad.retryable, Some(false));
        assert_eq!(bad.message, "the service answered 400");
    }

    #[test]
    fn null_is_a_failure_and_a_normal_answer_is_the_result() {
        let empty = interpret_response(&Value::Null).unwrap_err();
        assert_eq!(empty.code.as_deref(), Some("EMPTY_RESPONSE"));
        let fine = json!({ "status": 200, "hits": [1] });
        assert_eq!(interpret_response(&fine).unwrap(), fine);
    }
}
