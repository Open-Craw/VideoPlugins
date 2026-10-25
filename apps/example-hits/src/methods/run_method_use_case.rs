use serde_json::{json, Value};

use super::find_hits_algorithm::find_hits;
use super::measure_file_use_case::measure_file;

/// Runs a method of this plugin and gives its result, or the reason it failed.
///
/// - `find`: `{ durationMs, everyMs?, lengthMs?, label? }` gives `{ hits: [{ startMs, endMs, label }] }`.
/// - `size`: `{ audio }` gives the size of the stored file.
/// - `fail`: always an error. `crash`: ends the program without answering. `hang`: never answers.
///   The last three exist so a host can test how it copes with a misbehaving plugin.
pub fn run_method(method: &str, params: &Value) -> Result<Value, String> {
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
        let result = run_method("find", &json!({ "durationMs": 1_200 })).unwrap();
        assert_eq!(result["hits"].as_array().unwrap().len(), 2);
        assert_eq!(result["hits"][0]["label"], "hit");

        assert_eq!(
            run_method("nope", &json!({})).unwrap_err(),
            "no method \"nope\""
        );
        assert_eq!(
            run_method("fail", &json!({})).unwrap_err(),
            "this plugin always fails"
        );
    }
}
