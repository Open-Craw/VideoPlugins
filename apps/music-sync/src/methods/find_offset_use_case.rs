use serde_json::{json, Value};
use videoplugins_plugin_sdk::{number, whole_number, Cancel, Failure};
use videoplugins_sound_reader::{load_sound_input, Sound};

use crate::alignment::{find_alignment, measure_drift_ms, AlignmentOptions};

/// The rate both sounds are listened at: enough to tell a song by its notes and rhythm, small
/// enough to compare half an hour of sound in memory.
const LISTEN_RATE: u32 = 8_000;
/// The longest sound it compares, in milliseconds.
const LONGEST_MS: u64 = 30 * 60 * 1000;

/// The `findOffset` action: where `music` starts on the timeline of `video`'s sound.
///
/// Params: `video` and `music` (both sounds, or a video for its sound), `minScoreSigma` (2 to 100,
/// default 30: how far the best match must stand out from chance, below it the call fails with 422)
/// and `minOverlapMs` (default 5000: the least the two must share). It gives `offsetMs` (how late
/// the music starts on the video's timeline; negative when the video starts partway into the
/// music), `score`, `overlapMs` and `driftMs` (how much the offset changes from the start to the
/// end of what they share, null when it cannot be told).
pub fn find_music_offset(params: &Value, cancel: &Cancel) -> Result<Value, Failure> {
    let min_score = number(params, "minScoreSigma", 30.0, 2.0, 100.0)?;
    let min_overlap_ms = whole_number(params, "minOverlapMs", 5_000, 500, LONGEST_MS)?;
    for key in ["video", "music"] {
        if params.get(key).is_none_or(Value::is_null) {
            return Err(Failure::status(400, format!("{key} is missing")));
        }
    }
    let video = listen(params, "video")?;
    let music = listen(params, "music")?;
    for (name, sound) in [("video", &video), ("music", &music)] {
        if sound.duration_ms() > LONGEST_MS {
            return Err(Failure::status(
                413,
                format!("the {name} is longer than 30 minutes, which is the most this compares"),
            ));
        }
    }
    cancel.progress("comparing");
    let options = AlignmentOptions {
        sample_rate: LISTEN_RATE,
        min_overlap: (min_overlap_ms * u64::from(LISTEN_RATE) / 1000) as usize,
    };
    let found = find_alignment(&video.samples, &music.samples, options).ok_or_else(|| {
        Failure::status(
            422,
            "the sounds are too short or too quiet to compare (are they silent?)",
        )
    })?;
    if cancel.is_cancelled() {
        return Err(Failure::named("CANCELLED", "cancelled"));
    }
    if found.score < min_score {
        return Err(Failure::status(
            422,
            format!(
                "no match: the best one stands out {:.1} times the spread of chance, less than the {min_score} asked for. Is this the same music?",
                found.score
            ),
        )
        .with_details(json!({ "score": found.score, "offsetMs": found.offset_ms(LISTEN_RATE) })));
    }
    cancel.progress("checking the speed");
    let rate = LISTEN_RATE as usize;
    let drift_ms = measure_drift_ms(
        &video.samples,
        &music.samples,
        found.delay,
        LISTEN_RATE,
        4 * rate,
        2 * rate,
    );
    let shared_start = found.delay.max(0);
    let shared_end = (found.delay + music.samples.len() as isize).min(video.samples.len() as isize);
    Ok(json!({
        "offsetMs": found.offset_ms(LISTEN_RATE),
        "score": (found.score * 10.0).round() / 10.0,
        "overlapMs": (shared_end - shared_start).max(0) as u64 * 1000 / u64::from(LISTEN_RATE),
        "driftMs": drift_ms.map(|ms| ms.round()),
    }))
}

/// One of the two sounds, at the rate it is compared at.
fn listen(params: &Value, key: &str) -> Result<Sound, Failure> {
    load_sound_input(&json!({ "audio": params[key], "sampleRate": LISTEN_RATE }))
        .map_err(|error| Failure::status(error.code, format!("{key}: {}", error.message)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_missing_sound_is_a_400_that_names_it() {
        let failure =
            find_music_offset(&json!({ "video": "a.wav" }), &Cancel::default()).unwrap_err();
        assert_eq!(failure.code, Some(json!(400)));
        assert!(failure.message.contains("music is missing"));
    }

    #[test]
    fn params_out_of_range_are_a_400() {
        let failure =
            find_music_offset(&json!({ "minScoreSigma": 1 }), &Cancel::default()).unwrap_err();
        assert_eq!(failure.code, Some(json!(400)));
    }
}
