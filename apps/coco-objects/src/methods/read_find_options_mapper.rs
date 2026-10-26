use serde_json::Value;
use videoplugins_plugin_sdk::{number, text, whole_number, Failure};

use crate::classes::select_classes;
use crate::models::{model_named, MODELS};

use super::find_options_contract::FindOptions;

/// The params of `find` as options: `classes` (names; all when left out), `model` (`nano` by
/// default), `minConfidence` (0.05 to 1, default 0.5), `gapMs` (0 to ten minutes, default 2000) and
/// `minSamples` (1 to 100, default 2). Anything outside its range is a 400 that says what was wrong.
pub fn read_find_options(params: &Value) -> Result<FindOptions, Failure> {
    let name = text(params, "model", Some("nano"), 20)?.unwrap_or_default();
    let model = model_named(&name).ok_or_else(|| {
        let known: Vec<&str> = MODELS.iter().map(|model| model.name).collect();
        Failure::status(
            400,
            format!("model must be one of {}, not {name:?}", known.join(", ")),
        )
    })?;
    Ok(FindOptions {
        model,
        wanted: select_classes(params)?,
        min_confidence: number(params, "minConfidence", 0.5, 0.05, 1.0)? as f32,
        gap_ms: whole_number(params, "gapMs", 2_000, 0, 600_000)?,
        min_samples: whole_number(params, "minSamples", 2, 1, 100)? as usize,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn nothing_given_means_the_defaults() {
        let options = read_find_options(&json!({})).unwrap();
        assert_eq!(options.model.name, "nano");
        assert!(options.wanted.iter().all(|wanted| *wanted));
        assert_eq!(options.min_confidence, 0.5);
        assert_eq!((options.gap_ms, options.min_samples), (2_000, 2));
    }

    #[test]
    fn what_is_given_is_used_and_what_is_wrong_is_a_400_that_says_so() {
        let options = read_find_options(
            &json!({ "model": "s", "classes": ["cat"], "minConfidence": 0.7, "gapMs": 0, "minSamples": 1 }),
        )
        .unwrap();
        assert_eq!(options.model.input_size, 640);
        assert_eq!(options.wanted.iter().filter(|wanted| **wanted).count(), 1);

        for bad in [
            json!({ "model": "huge" }),
            json!({ "minConfidence": 2 }),
            json!({ "gapMs": -1 }),
            json!({ "minSamples": 0 }),
            json!({ "classes": ["unicorn"] }),
        ] {
            let failure = read_find_options(&bad).unwrap_err();
            assert_eq!(failure.code, Some(json!(400)), "{bad}");
        }
    }
}
