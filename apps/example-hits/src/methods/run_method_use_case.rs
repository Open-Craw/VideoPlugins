use serde_json::{json, Value};

use super::find_hits_algorithm::find_hits;
use super::measure_file_use_case::measure_file;
use super::wait_use_case::wait;

/// Runs a method of this plugin and gives its result, or the reason it failed.
///
/// - `find`: `{ durationMs, everyMs?, lengthMs?, label? }` gives `{ hits: [{ startMs, endMs, label }] }`.
/// - `size`: `{ audio }` gives the size of the stored file.
/// - `sleep`: `{ ms }` waits that long, or ends with the error `cancelled` as soon as the call is
///   cancelled. `pid` gives the number of the program's process, so a host can tell copies apart.
/// - `fail`: always an error. `crash`: ends the program without answering. `hang`: never answers
///   and ignores a cancel. The last three exist so a host can test how it copes with a
///   misbehaving plugin.
///
/// `is_cancelled` says whether the host cancelled this call.
pub fn run_method(
    method: &str,
    params: &Value,
    is_cancelled: &dyn Fn() -> bool,
) -> Result<Value, String> {
    match method {
        "find" => {
            let hits = find_hits(
                params["durationMs"].as_u64().unwrap_or(0),
                params["everyMs"].as_u64().unwrap_or(1_000),
                params["lengthMs"].as_u64().unwrap_or(500),
                params["label"].as_str().unwrap_or("hit"),
            );
            Ok(json!({ "hits": hits }))
        }
        "size" => measure_file(params),
        "sleep" => wait(params, is_cancelled),
        "pid" => Ok(json!({ "pid": std::process::id() })),
        "fail" => Err("this plugin always fails".to_string()),
        "crash" => std::process::exit(3),
        "hang" => {
            std::thread::sleep(std::time::Duration::from_secs(60));
            Ok(json!({}))
        }
        other => Err(format!("no method {other:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_uses_defaults_and_unknown_methods_are_errors() {
        let result = run_method("find", &json!({ "durationMs": 1_200 }), &|| false).unwrap();
        assert_eq!(result["hits"].as_array().unwrap().len(), 2);
        assert_eq!(result["hits"][0]["label"], "hit");

        assert_eq!(
            run_method("nope", &json!({}), &|| false).unwrap_err(),
            "no method \"nope\""
        );
        assert_eq!(
            run_method("fail", &json!({}), &|| false).unwrap_err(),
            "this plugin always fails"
        );
    }
}
