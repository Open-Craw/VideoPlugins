use serde_json::Value;
use videoplugins_plugin_sdk::{flag, number, whole_number, Failure};

use crate::classes::select_labels;

use super::find_options_contract::FindOptions;

/// The params of `find` as options: `labels` (names; all when left out), `minConfidence` (0.05 to
/// 1, default 0.5), `gapMs` (0 to ten minutes, default 1000), `minFrames` (1 to 100, default 1),
/// `clips` (default true), `clipPadMs` (0 to 5000, default 250) and `maxClipMs` (1000 to a minute,
/// default 10000). Anything outside its range is a 400 that says what was wrong.
pub fn read_find_options(params: &Value) -> Result<FindOptions, Failure> {
    Ok(FindOptions {
        wanted: select_labels(params)?,
        min_confidence: number(params, "minConfidence", 0.5, 0.05, 1.0)? as f32,
        gap_ms: whole_number(params, "gapMs", 1_000, 0, 600_000)?,
        min_frames: whole_number(params, "minFrames", 1, 1, 100)? as usize,
        clips: flag(params, "clips", true)?,
        clip_pad_ms: whole_number(params, "clipPadMs", 250, 0, 5_000)?,
        max_clip_ms: whole_number(params, "maxClipMs", 10_000, 1_000, 60_000)?,
    })
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn nothing_given_means_the_defaults() {
        let options = read_find_options(&json!({})).unwrap();
        assert!(options.wanted.iter().all(|wanted| *wanted));
        assert_eq!(options.min_confidence, 0.5);
        assert_eq!((options.gap_ms, options.min_frames), (1_000, 1));
        assert!(options.clips);
        assert_eq!((options.clip_pad_ms, options.max_clip_ms), (250, 10_000));
    }

    #[test]
    fn what_is_given_is_used_and_what_is_wrong_is_a_400_that_says_so() {
        let options = read_find_options(
            &json!({ "labels": ["Meow"], "minConfidence": 0.3, "clips": false, "gapMs": 0 }),
        )
        .unwrap();
        assert_eq!(options.wanted.iter().filter(|wanted| **wanted).count(), 1);
        assert!(!options.clips);

        for bad in [
            json!({ "minConfidence": 2 }),
            json!({ "gapMs": -1 }),
            json!({ "minFrames": 0 }),
            json!({ "clips": "yes" }),
            json!({ "maxClipMs": 10 }),
            json!({ "labels": ["Unicorn"] }),
        ] {
            let failure = read_find_options(&bad).unwrap_err();
            assert_eq!(failure.code, Some(json!(400)), "{bad}");
        }
    }
}
