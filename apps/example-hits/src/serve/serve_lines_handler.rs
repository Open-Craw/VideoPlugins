use std::io::{BufRead, Write};

use serde_json::{json, Value};

use crate::methods::run_method;

/// The protocol version this plugin speaks.
const PROTOCOL: u64 = 1;

/// Reads one JSON message per line from `input` and answers on `output`, until `shutdown` or the
/// end of the input. Lines that are not JSON, and messages of a type it does not know, are skipped.
pub fn serve_lines(input: impl BufRead, mut output: impl Write) {
    for line in input.lines().map_while(Result::ok) {
        let Ok(message) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let reply = match message["type"].as_str() {
            Some("hello") => json!({ "type": "ready", "protocol": PROTOCOL }),
            Some("call") => answer_call(&message),
            Some("shutdown") => return,
            _ => continue,
        };
        // Flush after every line: the host is waiting for it.
        if writeln!(output, "{reply}")
            .and_then(|()| output.flush())
            .is_err()
        {
            return;
        }
    }
}

fn answer_call(call: &Value) -> Value {
    let id = call["id"].clone();
    let method = call["method"].as_str().unwrap_or_default();
    match run_method(method, &call["params"]) {
        Ok(result) => json!({ "type": "result", "id": id, "result": result }),
        Err(message) => json!({ "type": "error", "id": id, "message": message }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn talk(lines: &[&str]) -> Vec<Value> {
        let input = lines.join("\n");
        let mut output = Vec::new();
        serve_lines(input.as_bytes(), &mut output);
        String::from_utf8(output)
            .unwrap()
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect()
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
        assert_eq!(replies[0]["type"], "error");
        assert_eq!(replies[0]["id"], 1);
        assert_eq!(replies[1]["type"], "result");
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
