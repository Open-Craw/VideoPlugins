use serde_json::Value;
use videoplugins_plugin_sdk::{flag, number, whole_number, Failure};

use super::extract_options_contract::ExtractOptions;

/// The params of `extract` as options: `keep` (0 to 1, default 0), `sampleRate` (16000 by default,
/// or 48000), `rest` (default true) and `files` (default false). Anything else is a 400 that says
/// what was wrong.
pub fn read_extract_options(params: &Value) -> Result<ExtractOptions, Failure> {
    let sample_rate = whole_number(params, "sampleRate", 16_000, 16_000, 48_000)? as u32;
    if sample_rate != 16_000 && sample_rate != 48_000 {
        return Err(Failure::status(
            400,
            format!("sampleRate must be 16000 or 48000, not {sample_rate}"),
        ));
    }
    Ok(ExtractOptions {
        keep: number(params, "keep", 0.0, 0.0, 1.0)? as f32,
        sample_rate,
        rest: flag(params, "rest", true)?,
        files: flag(params, "files", false)?,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn nothing_given_means_the_defaults() {
        let options = read_extract_options(&json!({})).unwrap();
        assert_eq!(options.keep, 0.0);
        assert_eq!(options.sample_rate, 16_000);
        assert!(options.rest && !options.files);
    }

    #[test]
    fn what_is_given_is_used_and_what_is_wrong_is_a_400_that_says_so() {
        let options = read_extract_options(
            &json!({ "keep": 0.15, "sampleRate": 48000, "rest": false, "files": true }),
        )
        .unwrap();
        assert_eq!((options.keep, options.sample_rate), (0.15, 48_000));
        assert!(!options.rest && options.files);
        for bad in [
            json!({ "keep": 2 }),
            json!({ "sampleRate": 44100 }),
            json!({ "sampleRate": 8000 }),
            json!({ "rest": "no" }),
            json!({ "files": 1 }),
        ] {
            let failure = read_extract_options(&bad).unwrap_err();
            assert_eq!(failure.code, Some(json!(400)), "{bad}");
        }
    }
}
