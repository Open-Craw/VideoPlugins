use std::collections::HashSet;
use std::io::{BufRead, Write};
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;

use serde_json::{json, Value};

use crate::methods::run_method;

/// The protocol version this plugin speaks.
const PROTOCOL: u64 = 1;

/// Reads one JSON message per line from `input` and answers on `output`, until `shutdown` or the
/// end of the input. Lines that are not JSON, and messages of a type it does not know, are skipped.
///
/// Every call runs on its own thread, so the plugin takes as many calls at once as the host sends
/// (its manifest says how many that is). A `cancel` tells the call with that id to stop. At the end
/// of the input the calls still running are waited for; at `shutdown` they are abandoned.
pub fn serve_lines(input: impl BufRead, output: impl Write + Send + 'static) {
    let output = Arc::new(Mutex::new(output));
    let cancelled: Arc<Mutex<HashSet<u64>>> = Arc::new(Mutex::new(HashSet::new()));
    let mut calls = Vec::new();

    for line in input.lines().map_while(Result::ok) {
        let Ok(message) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        match message["type"].as_str() {
            Some("hello") => {
                if !send(&output, &json!({ "type": "ready", "protocol": PROTOCOL })) {
                    return;
                }
            }
            Some("call") => {
                let output = Arc::clone(&output);
                let cancelled = Arc::clone(&cancelled);
                calls.push(thread::spawn(move || {
                    send(&output, &answer_call(&message, &cancelled));
                }));
            }
            Some("cancel") => {
                if let Some(id) = message["id"].as_u64() {
                    lock(&cancelled).insert(id);
                }
            }
            Some("shutdown") => return,
            _ => {}
        }
    }
    for call in calls {
        let _ = call.join();
    }
}

fn answer_call(call: &Value, cancelled: &Arc<Mutex<HashSet<u64>>>) -> Value {
    let id = call["id"].clone();
    let numeric_id = id.as_u64();
    let is_cancelled = || numeric_id.is_some_and(|id| lock(cancelled).contains(&id));
    let method = call["method"].as_str().unwrap_or_default();
    match run_method(method, &call["params"], &is_cancelled) {
        Ok(result) => json!({ "type": "result", "id": id, "result": result }),
        Err(failure) => {
            let mut reply = json!({ "type": "error", "id": id, "message": failure.message });
            // Only what the method said: the host treats the rest as not given.
            if let Some(code) = failure.code {
                reply["code"] = json!(code);
            }
            if let Some(retryable) = failure.retryable {
                reply["retryable"] = json!(retryable);
            }
            if let Some(details) = failure.details {
                reply["details"] = details;
            }
            reply
        }
    }
}

/// Writes one line and flushes: the host is waiting for it. False when the host is gone.
fn send<W: Write>(output: &Arc<Mutex<W>>, reply: &Value) -> bool {
    let mut output = lock(output);
    writeln!(output, "{reply}")
        .and_then(|()| output.flush())
        .is_ok()
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A writer the test can read after the plugin has used it.
    #[derive(Clone, Default)]
    struct Shared(Arc<Mutex<Vec<u8>>>);

    impl Write for Shared {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            lock(&self.0).extend_from_slice(bytes);
            Ok(bytes.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn talk(lines: &[&str]) -> Vec<Value> {
        let input = lines.join("\n");
        let output = Shared::default();
        serve_lines(input.as_bytes(), output.clone());
        let text = String::from_utf8(lock(&output.0).clone()).unwrap();
        text.lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
    }

    fn with_id(replies: &[Value], id: u64) -> &Value {
        replies
            .iter()
            .find(|reply| reply["id"] == id)
            .unwrap_or_else(|| panic!("no reply for call {id}"))
    }

    #[test]
    fn answers_hello_with_ready_and_a_call_with_its_result_under_the_same_id() {
        let replies = talk(&[
            r#"{"type":"hello","protocol":1}"#,
            r#"{"type":"call","id":7,"method":"find","params":{"durationMs":2500}}"#,
        ]);
        assert_eq!(replies[0], json!({ "type": "ready", "protocol": 1 }));
        assert_eq!(replies[1]["type"], "result");
        assert_eq!(replies[1]["id"], 7);
        assert_eq!(replies[1]["result"]["hits"].as_array().unwrap().len(), 3);
    }

    #[test]
    fn a_failing_call_is_an_error_with_the_same_id_and_the_plugin_goes_on() {
        let replies = talk(&[
            r#"{"type":"call","id":1,"method":"fail","params":{}}"#,
            r#"{"type":"call","id":2,"method":"find","params":{"durationMs":10}}"#,
        ]);
        assert_eq!(with_id(&replies, 1)["type"], "error");
        assert_eq!(with_id(&replies, 2)["type"], "result");
    }

    #[test]
    fn an_error_carries_the_code_the_retry_hint_and_the_details_the_method_gave() {
        let replies = talk(&[
            r#"{"type":"call","id":1,"method":"interpret","params":{"response":{"status":200,"error":true,"code":"QUOTA"}}}"#,
            r#"{"type":"call","id":2,"method":"interpret","params":{"response":{"status":200,"hits":[]}}}"#,
            r#"{"type":"call","id":3,"method":"nope","params":{}}"#,
            r#"{"type":"call","id":4,"method":"interpret","params":{"response":{"status":429}}}"#,
        ]);
        let flagged = with_id(&replies, 1);
        assert_eq!(flagged["type"], "error");
        assert_eq!(flagged["code"], "QUOTA");
        assert_eq!(flagged["details"]["status"], 200);
        assert_eq!(with_id(&replies, 2)["type"], "result");
        // A numeric code goes out as a number, and a failure that said nothing about retrying
        // sends no hint: the host works it out from the code.
        let unknown = with_id(&replies, 3);
        assert_eq!(unknown["code"], 404);
        assert!(unknown.get("retryable").is_none() && unknown.get("details").is_none());
        assert_eq!(with_id(&replies, 4)["code"], 429);
    }

    #[test]
    fn calls_run_at_the_same_time_and_a_cancel_stops_only_its_call() {
        let started = std::time::Instant::now();
        let replies = talk(&[
            r#"{"type":"call","id":1,"method":"sleep","params":{"ms":300}}"#,
            r#"{"type":"call","id":2,"method":"sleep","params":{"ms":300}}"#,
            r#"{"type":"call","id":3,"method":"sleep","params":{"ms":30000}}"#,
            r#"{"type":"cancel","id":3}"#,
        ]);
        // Two sleeps of 300 ms side by side take about 300 ms, not 600, and the long one ends early.
        assert!(started.elapsed().as_millis() < 5_000);
        assert_eq!(with_id(&replies, 1)["type"], "result");
        assert_eq!(with_id(&replies, 2)["type"], "result");
        assert_eq!(with_id(&replies, 3)["type"], "error");
        assert_eq!(with_id(&replies, 3)["message"], "cancelled");
    }

    #[test]
    fn stops_at_shutdown_and_skips_what_it_does_not_understand() {
        let replies = talk(&[
            "not json",
            r#"{"type":"mystery"}"#,
            r#"{"type":"shutdown"}"#,
            r#"{"type":"hello","protocol":1}"#,
        ]);
        assert!(replies.is_empty());
    }
}
